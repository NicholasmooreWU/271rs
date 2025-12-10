use anyhow::{bail, Context, Result};
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use hex::encode as hex_encode;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Simple SCM - minimal commit / add / revert system
#[derive(Parser)]
#[command(name = "scm")]
#[command(about = "Simple content-addressed SCM (Rust)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new repository in current directory (.scm)
    Init,
    /// Stage a file for commit
    Add {
        /// path to file
        path: PathBuf,
    },
    /// Commit staged files with message
    Commit {
        /// commit message
        #[arg(short, long)]
        message: String,
    },
    /// Revert working copy to parent of HEAD
    Revert,
    /// View commit logs
    Log,
    /// Show status (staged / modified)
    Status,
    /// Show diff between working copy and HEAD
    Diff {
        /// File to diff (optional)
        file: Option<PathBuf>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Commit {
    tree: BTreeMap<String, String>,
    parent: Option<String>,
    message: String,
    timestamp: DateTime<Utc>,
}

const SCM_DIR: &str = ".scm";
const OBJECTS_DIR: &str = ".scm/objects";
const COMMITS_DIR: &str = ".scm/commits";
const INDEX_FILE: &str = ".scm/index";
const HEAD_FILE: &str = ".scm/HEAD";

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => cmd_init(),
        Commands::Add { path } => cmd_add(&path),
        Commands::Commit { message } => cmd_commit(&message),
        Commands::Revert => cmd_revert(),
        Commands::Log => cmd_log(),
        Commands::Status => cmd_status(),
        Commands::Diff { file } => cmd_diff(file),
    }
}

fn cmd_init() -> Result<()> {
    if Path::new(SCM_DIR).exists() {
        println!("Already initialized.");
        return Ok(());
    }
    fs::create_dir(SCM_DIR)?;
    fs::create_dir(OBJECTS_DIR)?;
    fs::create_dir(COMMITS_DIR)?;
    fs::write(INDEX_FILE, "")?;
    fs::write(HEAD_FILE, "")?;
    println!("Initialized empty SCM repository in {}", SCM_DIR);
    Ok(())
}

fn repo_root() -> Result<std::path::PathBuf> {
    let mut cur = std::env::current_dir()?;
    loop {
        if cur.join(SCM_DIR).exists() {
            return Ok(cur);
        }
        if !cur.pop() {
            break;
        }
    }
    bail!("Not inside an scm repository (no {}). Run `scm init`", SCM_DIR);
}

fn load_index(repo: &Path) -> Result<Vec<String>> {
    let p = repo.join(INDEX_FILE);
    if !p.exists() {
        return Ok(vec![]);
    }
    let s = fs::read_to_string(p)?;
    let mut out = vec![];
    for line in s.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            out.push(trimmed.to_string());
        }
    }
    Ok(out)
}

fn write_index(repo: &Path, entries: &[String]) -> Result<()> {
    let p = repo.join(INDEX_FILE);
    let mut s = String::new();
    for e in entries {
        s.push_str(e);
        s.push('\n');
    }
    fs::write(p, s)?;
    Ok(())
}

fn cmd_add(path: &Path) -> Result<()> {
    let repo_root = repo_root()?;
    let repo = repo_root.as_path();
    let p = fs::canonicalize(path).with_context(|| format!("Can't canonicalize {:?}", path))?;
    if !p.exists() {
        bail!("File {:?} does not exist", path);
    }
    let repo_abs = fs::canonicalize(repo)?;
    let rel = pathdiff::diff_paths(&p, &repo_abs)
        .ok_or_else(|| anyhow::anyhow!("Path not inside repo"))?;
    let rel_str = rel.to_str().context("invalid utf8 path")?.to_string();

    let mut index = load_index(repo)?;
    if !index.contains(&rel_str) {
        index.push(rel_str.clone());
        index.sort();
    }
    write_index(repo, &index)?;
    println!("Added {}", rel_str);
    Ok(())
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex_encode(hasher.finalize())
}

fn store_blob(repo: &Path, bytes: &[u8]) -> Result<String> {
    let h = hash_bytes(bytes);
    let obj_path = repo.join(OBJECTS_DIR).join(&h);
    if !obj_path.exists() {
        fs::write(&obj_path, bytes)?;
    }
    Ok(h)
}

fn commit_object_hash(commit: &Commit) -> Result<String> {
    let json = serde_json::to_vec(commit)?;
    Ok(hash_bytes(&json))
}

fn read_head(repo: &Path) -> Result<Option<String>> {
    let p = repo.join(HEAD_FILE);
    let s = fs::read_to_string(p)?;
    let trimmed = s.trim();
    if trimmed.is_empty() {
        Ok(None)
    } else {
        Ok(Some(trimmed.to_string()))
    }
}

fn write_head(repo: &Path, hash: &str) -> Result<()> {
    let p = repo.join(HEAD_FILE);
    fs::write(p, hash)?;
    Ok(())
}

fn cmd_commit(message: &str) -> Result<()> {
    let repo_root = repo_root()?;
    let repo = repo_root.as_path();
    let index = load_index(repo)?;
    if index.is_empty() {
        bail!("Nothing staged. Use `scm add <file>` to stage files.");
    }
    let mut tree = BTreeMap::new();
    for rel in &index {
        let file_path = repo.join(rel);
        if !file_path.exists() {
            bail!("Staged file missing: {}", rel);
        }
        let mut f = fs::File::open(&file_path)?;
        let mut bytes = vec![];
        f.read_to_end(&mut bytes)?;
        let blob_hash = store_blob(repo, &bytes)?;
        tree.insert(rel.clone(), blob_hash);
    }

    let parent = read_head(repo)?;
    let commit = Commit {
        tree,
        parent,
        message: message.to_string(),
        timestamp: Utc::now(),
    };
    let commit_hash = commit_object_hash(&commit)?;
    let commit_path = repo.join(COMMITS_DIR).join(&commit_hash);
    let commit_json = serde_json::to_vec_pretty(&commit)?;
    fs::write(&commit_path, &commit_json)?;
    write_head(repo, &commit_hash)?;
    write_index(repo, &[])?;
    println!("Committed: {}", commit_hash);
    Ok(())
}

fn load_commit(repo: &Path, hash: &str) -> Result<Commit> {
    let path = repo.join(COMMITS_DIR).join(hash);
    if !path.exists() {
        bail!("Commit {} not found", hash);
    }
    let s = fs::read_to_string(path)?;
    let c: Commit = serde_json::from_str(&s)?;
    Ok(c)
}

fn checkout_commit(repo: &Path, commit_hash: &str) -> Result<()> {
    let commit = load_commit(repo, commit_hash)?;
    for (rel, blob_hash) in commit.tree.iter() {
        let blob_path = repo.join(OBJECTS_DIR).join(blob_hash);
        if !blob_path.exists() {
            bail!("Missing object {} for file {}", blob_hash, rel);
        }
        let bytes = fs::read(&blob_path)?;
        let target = repo.join(rel);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&target, &bytes)?;
    }
    Ok(())
}

fn cmd_revert() -> Result<()> {
    let repo_root = repo_root()?;
    let repo = repo_root.as_path();
    let head = read_head(repo)?;
    let head = match head {
        Some(h) => h,
        None => bail!("No commits to revert."),
    };
    let commit = load_commit(repo, &head)?;
    let parent = match commit.parent {
        Some(ref p) => p.clone(),
        None => bail!("No parent commit to revert to."),
    };
    let _parent_commit = load_commit(repo, &parent)?;
    checkout_commit(repo, &parent)?;
    write_head(repo, &parent)?;
    println!("Reverted HEAD {} -> {}", head, parent);
    Ok(())
}

fn cmd_log() -> Result<()> {
    let repo_root = repo_root()?;
    let repo = repo_root.as_path();
    let mut cur = read_head(repo)?;
    while let Some(h) = cur {
        let c = load_commit(repo, &h)?;
        println!("commit {}\nDate:   {}\n\n    {}\n", h, c.timestamp.to_rfc3339(), c.message);
        cur = c.parent;
    }
    Ok(())
}

fn file_content_hash(repo: &Path, rel: &str) -> Result<Option<String>> {
    let p = repo.join(rel);
    if !p.exists() {
        return Ok(None);
    }
    let b = fs::read(&p)?;
    Ok(Some(hash_bytes(&b)))
}

fn cmd_status() -> Result<()> {
    let repo_root = repo_root()?;
    let repo = repo_root.as_path();
    let staged = load_index(repo)?;
    println!("Staged files:");
    for s in &staged {
        println!("  {}", s);
    }
    let head = read_head(repo)?;
    if head.is_none() {
        println!("No commits yet.");
        return Ok(());
    }
    let head = head.unwrap();
    let commit = load_commit(repo, &head)?;
    println!("\nTracked files (in HEAD):");
    for (path, blob_hash) in commit.tree.iter() {
        let cur_hash = file_content_hash(repo, path)?;
        match cur_hash {
            None => println!("  {} (deleted)", path),
            Some(h) if h == *blob_hash => println!("  {} (up-to-date)", path),
            Some(_) => println!("  {} (modified)", path),
        }
    }
    Ok(())
}

fn load_blob(repo: &Path, hash: &str) -> Result<Vec<u8>> {
    let p = repo.join(OBJECTS_DIR).join(hash);
    if !p.exists() {
        bail!("Object {} not found", hash);
    }
    let b = fs::read(p)?;
    Ok(b)
}

fn read_file_to_lines(path: &Path) -> Result<Vec<String>> {
    if !path.exists() {
        return Ok(vec![]);
    }
    let s = fs::read_to_string(path)?;
    Ok(s.lines().map(|l| l.to_string()).collect::<Vec<String>>())
}

fn cmd_diff(file: Option<PathBuf>) -> Result<()> {
    let repo_root = repo_root()?;
    let repo = repo_root.as_path();
    let head = read_head(repo)?;
    if head.is_none() {
        println!("No commits yet.");
        return Ok(());
    }
    let head = head.unwrap();
    let commit = load_commit(repo, &head)?;

    if let Some(fp) = file {
        let rel = pathdiff::diff_paths(&fs::canonicalize(fp)?, &fs::canonicalize(repo)?)
            .ok_or_else(|| anyhow::anyhow!("File not in repo"))?;
        let rels = rel.to_str().unwrap();
        let committed_blob = commit.tree.get(rels);
        let committed_lines: Vec<String> = if let Some(hash) = committed_blob {
            let bytes = load_blob(repo, hash)?;
            String::from_utf8_lossy(&bytes)
                .lines()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        } else {
            vec![]
        };
        let working = read_file_to_lines(&repo.join(rels))?;
        print_diff(&committed_lines, &working, rels);
    } else {
        for (path, hash) in commit.tree.iter() {
            let committed_lines: Vec<String> = {
                let bytes = load_blob(repo, hash)?;
                String::from_utf8_lossy(&bytes)
                    .lines()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            };
            let working = read_file_to_lines(&repo.join(path))?;
            if committed_lines != working {
                print_diff(&committed_lines, &working, path);
            }
        }
    }
    Ok(())
}

fn print_diff(a: &[String], b: &[String], label: &str) {
    use diff::lines;
    println!("Diff for {}\n", label);
    let a_joined = a.join("\n");
    let b_joined = b.join("\n");
    let diffs = lines(&a_joined, &b_joined);
    for d in diffs {
        match d {
            diff::Result::Left(l) => println!("- {}", l),
            diff::Result::Right(r) => println!("+ {}", r),
            diff::Result::Both(_, _) => {}
        }
    }
    println!();
}

