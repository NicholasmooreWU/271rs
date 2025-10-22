use std::env;
use bignum::{
    add_ix, sub_ix, mul_ix, div_ix, rem_ix, h2i_ix, see_ix,
};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: cargo run <hex1> <hex2> <OP>");
        std::process::exit(1);
    }

    let a_hex = &args[1];
    let b_hex = &args[2];
    let op = &args[3].to_uppercase();

    let a = h2i_ix(a_hex);
    let b = h2i_ix(b_hex);

    let result = match op.as_str() {
        "ADD" => add_ix(&a, &b),
        "SUB" => sub_ix(&a, &b),
        "MUL" => mul_ix(&a, &b),
        "QUO" | "DIV" => div_ix(&a, &b),
        "REM" => rem_ix(&a, &b),
        _ => {
            eprintln!("Unknown operation: {}", op);
            std::process::exit(1);
        }
    };

    see_ix(&result);
    println!();
}

