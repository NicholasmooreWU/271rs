// src/field.rs
use num_bigint::BigUint;
use num_traits::{One, Zero};

pub const B_BITS: usize = 256;

// q = 2^255 - 19
pub fn q() -> BigUint {
    // compute as BigUint to match Python constants
    // placeholder; implement properly
    let one = BigUint::one();
    (BigUint::one() << 255u32) - BigUint::from(19u32)
}

// l = 2^252 + 27742317777372353535851937790883648493
pub fn l() -> BigUint {
    (BigUint::one() << 252u32) + BigUint::parse_bytes(b"27742317777372353535851937790883648493", 10).unwrap()
}

/// Modular exponentiation: base^exp mod modulus
pub fn expmod(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    // iterative binary exponentiation
    let mut result = BigUint::one();
    let mut b = base % modulus;
    let mut e = exp.clone();

    while !e.is_zero() {
        if &e & BigUint::one() == BigUint::one() {
            result = (result * &b) % modulus;
        }
        b = (&b * &b) % modulus;
        e >>= 1;
    }
    result
}

/// multiplicative inverse: x^(q-2) mod q
pub fn inv(x: &BigUint) -> BigUint {
    let modulus = q();
    let exp = (&modulus) - BigUint::from(2u32);
    expmod(x, &exp, &modulus)
}

