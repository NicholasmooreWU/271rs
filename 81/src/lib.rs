// src/lib.rs
pub mod field;
pub mod point;
pub mod encode;
pub mod hash;

/// Public API similar to the Python reference:
/// - publickey(secret_key: &[u8]) -> Vec<u8>
/// - signature(message: &[u8], secret_key: &[u8], public_key: &[u8]) -> Vec<u8>
/// - check_valid(sig: &[u8], message: &[u8], public_key: &[u8]) -> bool
///
/// For now these are stubs to be implemented.
pub fn publickey(_sk: &[u8]) -> Vec<u8> {
    unimplemented!("implement publickey")
}

pub fn signature(_m: &[u8], _sk: &[u8], _pk: &[u8]) -> Vec<u8> {
    unimplemented!("implement signature")
}

pub fn check_valid(_s: &[u8], _m: &[u8], _pk: &[u8]) -> bool {
    unimplemented!("implement check_valid")
}

