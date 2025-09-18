pub fn weight_u8(byte: u8) -> u64 {
    let mut cnt : u64 = 0;
    let mut b = byte;
    for _ in 0..8 {
        cnt += (b & 1) as u64;
        b >>= 1;
    }
    return cnt;
}
pub fn weight_u64(word: u64) -> u64 {
    let mut cnt : u64 = 0;
    let mut w = word;
    for _ in 0..8 {
        let byte = (w & 0xFF) as u8;
        cnt += weight_u8(byte);
        w >>= 8;
    }
    return cnt;
}
pub fn weight_bytes(bytes: Vec<u8>) -> u64 {
    let mut cnt : u64 = 0;
    for b in bytes {
        cnt += weight_u8(b);
    }
    return cnt;
}
pub fn weight_words(words: Vec<u64>) -> u64 {
    let mut cnt : u64 = 0;
    for w in words {
        cnt += weight_u64(w);
    }
    return cnt;
}
pub fn distance_u8(b: u8, c: u8) -> u64 {
    let mut cnt : u64 = 0;
    let mut diff = b ^ c;
    for _ in 0..8 {
        cnt += (diff & 1) as u64;
        diff >>= 1;
    }
    return cnt;
}
pub fn distance_u64(w: u64, x: u64) -> u64 {
    let mut cnt : u64 = 0;
    let mut diff = w ^ x;
    for _ in 0..64 {
        cnt += (diff & 1) as u64;
        diff >>= 1;
    }
    return cnt;
}
pub fn distance_bytes(bs: Vec<u8>, cs: Vec<u8>) -> u64 {
    let mut cnt : u64 = 0;
    assert_eq!(bs.len(), cs.len(), "vectors must have the same length");
    for (b, c) in bs.into_iter().zip(cs.into_iter()) {
        cnt += distance_u8(b, c);
    }
    return cnt;
}
pub fn distance_words(ws: Vec<u64>, xs: Vec<u64>) -> u64 {
    let mut cnt : u64 = 0;
    assert_eq!(ws.len(), xs.len(), "vectors must have the same length");
    for (w, x) in ws.into_iter().zip(xs.into_iter()) {
        cnt += distance_u64(w, x);
    }
    return cnt;
}

