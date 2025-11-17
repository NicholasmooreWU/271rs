use std::io;
use dag::build_graph;
use std::collections::HashMap;

fn print_requirements(map: &HashMap<String, dag::Node>) {
    let mut courses: Vec<&String> = map.iter()
        .filter_map(|(k, v)| if !v.prereqs.is_empty() { Some(k) } else { None })
        .collect();
    courses.sort();

    for course in courses {
        let node = &map[course];
        println!("{} requires {}", node.name, node.prereqs.join(","));
    }
}

fn main() {
    println!("paste edges (PREREQ:COURSE), finish with empty line (or send EOF):");

    let stdin = io::stdin();
    let map = build_graph(stdin.lock());

    if map.is_empty() {
        println!("(no courses read)");
        return;
    }

    print_requirements(&map);
}

