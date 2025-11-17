// src/hash.rs
use sha2::{Digest, Sha512};

pub fn H(m: &[u8]) -> [u8; 64] {
    let mut hasher = Sha512::new();
    hasher.update(m);
    let result = hasher.finalize();
    let mut out = [0u8; 64];
    out.copy_from_slice(&result);
    out
}

/// Hint: convert SHA512 output bytes into a big integer per bit-ordering rules
pub fn hint(_m: &[u8]) -> num_bigint::BigUint {
    unimplemented!();
}

