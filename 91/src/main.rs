use std::io::{self, BufRead};

fn main() {
    println!("waiting for edges; end with an empty line");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.trim().is_empty() {
            println!("got empty line — finishing");
            break;
        }
        println!("read line: {}", l);
    }
    println!("done");
}

