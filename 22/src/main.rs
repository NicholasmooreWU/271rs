use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
const BG_GREEN: &str = "\u{001b}[42m";
const BG_YELLOW: &str = "\u{001b}[43m";
const BG_GRAY: &str = "\u{001b}[40m";
const WHT: &str = "\u{001b}[0m";
const TOP: &str    = "┌───┬───┬───┬───┬───┐";
const MID: &str    = "├───┼───┼───┼───┼───┤";
const BOTTOM: &str = "└───┴───┴───┴───┴───┘";

fn load_words<P: AsRef<Path>>(path: P) -> Vec<String> {
    let s = std::fs::read_to_string(path).expect("Failed to read words.txt");
    s.lines()
        .map(|l| l.trim().to_lowercase())
        .filter(|w| w.len() == 5)
        .collect()
}
fn print_letter_with_bg(ch: char, bg: &str) {
    print!("│ {}{}{} ", bg, ch, WHT);
}
fn render_board(guesses: &[String; 6], answer: &str) {
    print!("\u{001b}[2J");
    println!("{}", TOP);
    for row in 0..5 {
        let guess = &guesses[row];
        for (i, ch) in guess.chars().enumerate() {
            let color = if answer.chars().nth(i).unwrap_or(' ') == ch {
                BG_GREEN
            } else if answer.contains(ch) {
                BG_YELLOW
            } else {
                BG_GRAY
            };
            print_letter_with_bg(ch, color);
        }
        println!("│");
        println!("{}", MID);
    }
    let guess = &guesses[5];
    for (i, ch) in guess.chars().enumerate() {
        let color = if answer.chars().nth(i).unwrap_or(' ') == ch {
            BG_GREEN
        } else if answer.contains(ch) {
            BG_YELLOW
        } else {
            BG_GRAY
        };
        print_letter_with_bg(ch, color);
    }
    println!("│");
    println!("{}", BOTTOM);
}
fn pick_random_index_unix(len: usize) -> usize {
    let mut buffer = [0u8; (usize::BITS / 8) as usize];
    let mut devrnd = File::open("/dev/urandom").expect("Could not open /dev/urandom");
    std::io::Read::read_exact(&mut devrnd, &mut buffer).expect("Failed to read /dev/urandom");
    let rand_usize = usize::from_ne_bytes(buffer);
    rand_usize % len
}
fn main() {
    let words = load_words("words.txt");
    if words.is_empty() {
        eprintln!("words.txt is empty or missing valid 5-letter words.");
        return;
    }
    let secret_idx = pick_random_index_unix(words.len());
    let answer = words[secret_idx].clone();
    let mut guesses: [String; 6] = [
        "     ".to_string(),
        "     ".to_string(),
        "     ".to_string(),
        "     ".to_string(),
        "     ".to_string(),
        "     ".to_string(),
    ];
    let mut attempts = 0usize;
    render_board(&guesses, &answer);
    println!("(use lowercase; guesses must be in the word list)");
    loop {
        print!("Enter guess: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read stdin");
        let guess = input.trim().to_lowercase();
        if guess.len() != 5 {
            println!("Please enter exactly 5 letters.");
            continue;
        }
        if !words.contains(&guess) {
            println!("Not a valid word in the list.");
            continue;
        }
        if attempts < 6 {
            guesses[attempts] = guess.clone();
        }
        render_board(&guesses, &answer);
        attempts += 1;
        if guess == answer {
            println!("You win! The answer was `{}`.", answer);
            break;
        }
        if attempts >= 6 {
            println!("Out of guesses. The answer was `{}`.", answer);
            break;
        }
    }
}
