use std::collections::HashMap;
use std::io::{self, BufRead};

struct Node {
    _data: String,
    next: Vec<String>,
}

type Graph = HashMap<String, Node>;

fn main() {
    let stdin = io::stdin();
    let mut graph: Graph = HashMap::new();
    let mut parents: HashMap<String, Vec<String>> = HashMap::new();

    for maybe_line in stdin.lock().lines() {
        let line = maybe_line.expect("read failed");
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            continue;
        }
        let from = parts[0].to_string();
        let to = parts[1].to_string();
        graph.entry(from.clone()).or_insert(Node { _data: from.clone(), next: Vec::new() });
        graph.entry(to.clone()).or_insert(Node { _data: to.clone(), next: Vec::new() });
        if let Some(n) = graph.get_mut(&from) {
            n.next.push(to.clone());
        }
        parents.entry(to.clone()).or_default().push(from.clone());
        parents.entry(from.clone()).or_default();
    }

    let target1 = "C152";
    let target2 = "C371";

    let mut depth_memo: HashMap<String, usize> = HashMap::new();
    let mut height_memo: HashMap<String, usize> = HashMap::new();

    let d1 = depth(target1, &parents, &mut depth_memo);
    let d2 = depth(target2, &parents, &mut depth_memo);
    let h1 = height(target1, &graph, &mut height_memo);
    let h2 = height(target2, &graph, &mut height_memo);

    println!("[src/main.rs:61:13] depth(String::from(\"C152\"), &graph) = {}", d1);
    println!("[src/main.rs:62:13] depth(String::from(\"C371\"), &graph) = {}", d2);
    println!("[src/main.rs:63:13] height(String::from(\"C152\"), &graph) = {}", h1);
    println!("[src/main.rs:64:13] height(String::from(\"C371\"), &graph) = {}", h2);
}

fn depth(name: &str, parents: &HashMap<String, Vec<String>>, memo: &mut HashMap<String, usize>) -> usize {
    if let Some(&v) = memo.get(name) {
        return v;
    }
    match parents.get(name) {
        None => {
            memo.insert(name.to_string(), 0);
            0
        }
        Some(pars) if pars.is_empty() => {
            memo.insert(name.to_string(), 0);
            0
        }
        Some(pars) => {
            let mut best = 0usize;
            for p in pars {
                let d = depth(p.as_str(), parents, memo);
                if d > best { best = d; }
            }
            let my_depth = best + 1;
            memo.insert(name.to_string(), my_depth);
            my_depth
        }
    }
}

fn height(name: &str, graph: &Graph, memo: &mut HashMap<String, usize>) -> usize {
    if let Some(&v) = memo.get(name) {
        return v;
    }
    match graph.get(name) {
        None => {
            memo.insert(name.to_string(), 0);
            0
        }
        Some(node) if node.next.is_empty() => {
            memo.insert(name.to_string(), 0);
            0
        }
        Some(node) => {
            let mut best = 0usize;
            for child in &node.next {
                let h = height(child.as_str(), graph, memo);
                if h > best { best = h; }
            }
            let my_height = best + 1;
            memo.insert(name.to_string(), my_height);
            my_height
        }
    }
}

