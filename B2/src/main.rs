fn lcs(s1: &str, s2: &str) -> String {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    let m = a.len();
    let n = b.len();

    let mut dp: Vec<Vec<usize>> = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    let mut i = m;
    let mut j = n;
    let mut rev_res: Vec<char> = Vec::with_capacity(dp[m][n]);

    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            rev_res.push(a[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    rev_res.iter().rev().collect()
}

fn main() {
    let mut args = std::env::args();
    let _exe = args.next();

    let s1 = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Usage: cargo run -- <string1> <string2>");
            std::process::exit(1);
        }
    };

    let s2 = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Usage: cargo run -- <string1> <string2>");
            std::process::exit(1);
        }
    };

    dbg!(lcs(&s1, &s2));
}

