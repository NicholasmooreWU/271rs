// src/encode.rs

/// Helpers for encodeint, decodeint, encodepoint, decodepoint, bit extraction.
///
/// Keep careful: Python uses little-endian bit ordering inside bytes.
/// Implement these exactly to match the reference test vectors.
pub fn encodeint(_y: &num_bigint::BigUint) -> Vec<u8> {
    unimplemented!();
}

pub fn decodeint(_s: &[u8]) -> num_bigint::BigUint {
    unimplemented!();
}

pub fn encodepoint() -> Vec<u8> {
    unimplemented!();
}

