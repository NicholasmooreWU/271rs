use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};

fn encode_ascii85(data: &[u8]) -> String {
    let ascii85_chars: Vec<u8> = (33u8..=117u8).collect();

    let mut encoded = String::new();
    let mut i = 0;
    while i < data.len() {
        let mut tuple: u32 = 0;
        for j in 0..4 {
            tuple <<= 8;
            tuple |= data[i + j] as u32;
        }

        let mut t = tuple;
        let mut buffer = [0u8; 5];
        for j in (0..5).rev() {
            let rem = (t % 85) as usize;
            buffer[j] = ascii85_chars[rem];
            t /= 85;
        }

        encoded.push_str(std::str::from_utf8(&buffer[..5]).unwrap());
        i += 4;
    }

    encoded
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let mut file = File::open(&args[1])?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let encoded = encode_ascii85(&data);
    let mut stdout = io::stdout().lock();
    stdout.write_all(b"<~")?;
    let mut col = 2;

    for ch in encoded.bytes() {
        stdout.write_all(&[ch])?;
        col += 1;
        if col % 80 == 0 {
            stdout.write_all(b"\n")?;
        }
    }
    stdout.write_all(b"~>")?;
    stdout.flush()?;

    Ok(())
}

