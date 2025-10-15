use io::*; // use the current crate’s library

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: cargo run <hex1> <hex2> <OP>");
        return;
    }

    let a = h2i_ix(&args[1]);
    let b = h2i_ix(&args[2]);
    let op = args[3].as_str();

    match op {
        "ADD" => see_ix(&add_ix(&a, &b)),
        "SUB" => see_ix(&sub_ix(&a, &b)),
        "MUL" => see_ix(&mul_ix(&a, &b)),
        // accept both DIV and QUO (tester used QUO)
        "DIV" | "QUO" => see_ix(&div_ix(&a, &b)),
        "REM" | "MOD" => see_ix(&rem_ix(&a, &b)),
        _ => println!("Operator not recognized: choose from ADD, SUB, MUL, DIV, REM"),
    }
}
