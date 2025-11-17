// tests/vectors.rs
use std::fs;
use std::path::PathBuf;

#[test]
#[ignore] // remove ignore after implementation
fn test_vectors_from_signs_input() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push("signs.input");
    let txt = fs::read_to_string(path).expect("read signs.input");

    for (i, line) in txt.lines().enumerate() {
        if line.trim().is_empty() { continue; }
        // Parse fields: sk:pk:m:sm:
        let parts: Vec<&str> = line.split(':').collect();
        assert!(parts.len() >= 4, "malformed line {}", i);
        // For now we only check parsing — the cryptographic functions are unimplemented.
        // After implementing publickey/signature/check_valid, call them and compare bytes.
    }
}

