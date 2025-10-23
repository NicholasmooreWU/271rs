// src/main.rs
use sha_consts::{approx_frac_using_f64, compute_frac_bits_sqrt};

fn main() {
    let primes = [2u64,3,5,7,11,13,17,19];

    println!("prime |   approx(f64)    |    exact (BigInt binary-search)");
    for &p in primes.iter() {
        let a = approx_frac_using_f64(p);
        let e = compute_frac_bits_sqrt(p);
        println!("{:>5} | {:#018x} | {:#018x}", p, a, e);
    }
}

