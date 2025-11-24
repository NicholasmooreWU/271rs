use std::env;
use std::fs;
use std::process;

#[derive(Debug)]
enum Op {
    Equal(usize, usize),
    Delete(usize, String),
    Add(usize, String),
}

fn read_lines(fname: &str) -> Vec<String> {
    fs::read_to_string(fname)
        .unwrap_or_else(|e| {
            eprintln!("Failed to read {}: {}", fname, e);
            process::exit(1);
        })
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn build_lcs_table(left: &[String], right: &[String]) -> Vec<Vec<usize>> {
    let n = left.len();
    let m = right.len();
    let mut dp = vec![vec![0usize; m + 1]; n + 1];

    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if left[i] == right[j] {
                dp[i][j] = dp[i + 1][j + 1] + 1;
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }
    dp
}

fn backtrack_ops(left: &[String], right: &[String], dp: &Vec<Vec<usize>>) -> Vec<Op> {
    let mut ops: Vec<Op> = Vec::new();
    let mut i = 0usize;
    let mut j = 0usize;
    let n = left.len();
    let m = right.len();

    while i < n && j < m {
        if left[i] == right[j] {
            ops.push(Op::Equal(i + 1, j + 1));
            i += 1;
            j += 1;
        } else if dp[i + 1][j] >= dp[i][j + 1] {
            ops.push(Op::Delete(i + 1, left[i].clone()));
            i += 1;
        } else {
            ops.push(Op::Add(j + 1, right[j].clone()));
            j += 1;
        }
    }

    while i < n {
        ops.push(Op::Delete(i + 1, left[i].clone()));
        i += 1;
    }
    while j < m {
        ops.push(Op::Add(j + 1, right[j].clone()));
        j += 1;
    }

    ops
}

fn format_side(indices: &Vec<usize>, fallback: usize) -> String {
    match indices.len() {
        0 => format!("{}", fallback),
        1 => format!("{}", indices[0]),
        _ => {
            let first = indices[0];
            let last = *indices.last().unwrap();
            if first == last {
                format!("{}", first)
            } else {
                format!("{},{}", first, last)
            }
        }
    }
}

fn print_diff_like(ops: &[Op]) {
    let mut left_block: Vec<usize> = Vec::new();
    let mut right_block: Vec<usize> = Vec::new();
    let mut left_lines: Vec<String> = Vec::new();
    let mut right_lines: Vec<String> = Vec::new();

    let mut last_left_no: usize = 0;
    let mut last_right_no: usize = 0;

    let flush_block = |left_block: &Vec<usize>,
                       right_block: &Vec<usize>,
                       left_lines: &Vec<String>,
                       right_lines: &Vec<String>,
                       last_left_no: usize,
                       last_right_no: usize| {
        if left_block.is_empty() && right_block.is_empty() {
            return;
        }

        let op_char = match (left_block.is_empty(), right_block.is_empty()) {
            (false, true) => 'd',
            (true, false) => 'a',
            (false, false) => 'c',
            _ => '?',
        };

        let left_shorthand = format_side(left_block, last_left_no);
        let right_shorthand = format_side(right_block, last_right_no);

        println!("{}{}{}", left_shorthand, op_char, right_shorthand);

        for l in left_lines {
            println!("< {}", l);
        }

        if op_char == 'c' {
            println!("---");
        }

        for r in right_lines {
            println!("> {}", r);
        }
    };

    for op in ops {
        match op {
            Op::Equal(lno, rno) => {
                flush_block(
                    &left_block,
                    &right_block,
                    &left_lines,
                    &right_lines,
                    last_left_no,
                    last_right_no,
                );
                left_block.clear();
                right_block.clear();
                left_lines.clear();
                right_lines.clear();

                last_left_no = *lno;
                last_right_no = *rno;
            }
            Op::Delete(lno, line) => {
                left_block.push(*lno);
                left_lines.push(line.clone());
                last_left_no = *lno;
            }
            Op::Add(rno, line) => {
                right_block.push(*rno);
                right_lines.push(line.clone());
                last_right_no = *rno;
            }
        }
    }

    flush_block(
        &left_block,
        &right_block,
        &left_lines,
        &right_lines,
        last_left_no,
        last_right_no,
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} left.txt right.txt", args[0]);
        process::exit(1);
    }

    let left = read_lines(&args[1]);
    let right = read_lines(&args[2]);

    let dp = build_lcs_table(&left, &right);
    let ops = backtrack_ops(&left, &right, &dp);

    print_diff_like(&ops);
}

