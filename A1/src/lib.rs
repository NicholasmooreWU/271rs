use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub name: String,
    pub prereqs: Vec<String>,
}

/// Build a graph from lines of the form "PREREQ:COURSE".
/// Stops on the first empty line (line == "") or EOF.
/// Returns a HashMap course_name -> Node
pub fn build_graph<R: BufRead>(reader: R) -> HashMap<String, Node> {
    let mut map: HashMap<String, Node> = HashMap::new();

    for line_res in reader.lines() {
        let line = line_res.expect("read error");
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break; // terminate on empty line (also EOF will end loop)
        }
        if trimmed.starts_with('#') {
            continue; // allow comments
        }

        match trimmed.split_once(':') {
            Some((pre, course)) => {
                let pre = pre.trim().to_string();
                let course = course.trim().to_string();

                // ensure prereq node exists
                map.entry(pre.clone()).or_insert_with(|| Node {
                    name: pre.clone(),
                    prereqs: Vec::new(),
                });

                // ensure course node exists and push prereq (avoid duplicates)
                let course_node = map.entry(course.clone()).or_insert_with(|| Node {
                    name: course.clone(),
                    prereqs: Vec::new(),
                });

                if !course_node.prereqs.contains(&pre) {
                    course_node.prereqs.push(pre);
                }
            }
            None => {
                // skip malformed lines but warn
                eprintln!("warning: skipping malformed line: '{}'", trimmed);
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn builds_expected_graph() {
        let data = "\
C151:C280
C151:C152
C151:D351
C152:C271
C152:C351
C152:C262
C271:C371
C351:C480
C371:C480
C480:C481
M150:M251
M150:M280
M150:P221
M251:C351
M251:M352
P221:P222
";
        let cursor = Cursor::new(data);
        let map = build_graph(cursor);

        let c152 = map.get("C152").expect("C152 present");
        assert!(c152.prereqs.contains(&"C151".to_string()));

        let c480 = map.get("C480").expect("C480 present");
        assert_eq!(c480.prereqs.len(), 2);
        assert!(c480.prereqs.contains(&"C351".to_string()));
        assert!(c480.prereqs.contains(&"C371".to_string()));

        let m251 = map.get("M251").expect("M251 present");
        assert_eq!(m251.prereqs, vec!["M150".to_string()]);
    }
}
