use num_bigint::{BigInt, Sign};
use num_traits::{Zero, One, ToPrimitive, Euclid};
use num_integer::Integer;
use sha2::{Digest, Sha512};


fn h(m: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(m);
    hasher.finalize().to_vec()
}

fn bit(h_val: &[u8], i: usize) -> u8 {
    let byte = h_val[i / 8];
    ((byte >> (i % 8)) & 1) as u8
}

pub fn expmod(b_val: &BigInt, e: &BigInt, m: &BigInt) -> BigInt {
    let mut base = b_val.rem_euclid(m).clone();
    let mut exp = e.clone();
    let mut result = BigInt::one();
    while exp > BigInt::zero() {
        if (&exp & BigInt::one()) == BigInt::one() {
            result = (result * &base).rem_euclid(m);
        }
        base = (&base * &base).rem_euclid(m);
        exp >>= 1;
    }
    result
}

pub fn inv(x: &BigInt, q: &BigInt) -> BigInt {
    expmod(x, &(q - BigInt::from(2)), q)
}

pub fn xrecover(y: &BigInt, q: &BigInt, d: &BigInt, i_const: &BigInt) -> BigInt {
    let yy = (y * y).rem_euclid(q);
    let num = (yy.clone() - BigInt::one()).rem_euclid(q);
    let den = (d * &yy + BigInt::one()).rem_euclid(q);
    let xx = (num * inv(&den, q)).rem_euclid(q);
    let mut x = expmod(&xx, &((&q + BigInt::from(3)) / BigInt::from(8)), q);

    if (&x * &x - xx).rem_euclid(q) != BigInt::zero() {
        x = (x * i_const).rem_euclid(q);
    }
    if x.is_odd() {
        x = q - x;
    }
    x.rem_euclid(q)
}


fn edwards(p: &Vec<BigInt>, q_val: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    let (x1, y1) = (&p[0], &p[1]);
    let (x2, y2) = (&q_val[0], &q_val[1]);

    let x1y2 = (x1 * y2).rem_euclid(q);
    let x2y1 = (x2 * y1).rem_euclid(q);
    let x_top = (x1y2 + x2y1).rem_euclid(q);

    let y1y2 = (y1 * y2).rem_euclid(q);
    let x1x2 = (x1 * x2).rem_euclid(q);
    let dxx_yy = (d * x1x2 * y1y2).rem_euclid(q);

    let x_bot = (BigInt::one() + &dxx_yy).rem_euclid(q);
    let y_bot = (BigInt::one() - &dxx_yy).rem_euclid(q);

    let x3 = (x_top * inv(&x_bot, q)).rem_euclid(q);
    let y3 = ((y1y2 - x1x2) * inv(&y_bot, q)).rem_euclid(q);

    vec![x3, y3]
}

fn scalarmult(p: &Vec<BigInt>, e: &BigInt, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    let mut n = e.clone();
    let mut q_point = vec![BigInt::zero(), BigInt::one()];
    let mut base = p.clone();

    while n > BigInt::zero() {
        if (&n & BigInt::one()) == BigInt::one() {
            q_point = edwards(&q_point, &base, q, d);
        }
        base = edwards(&base, &base, q, d);
        n >>= 1;
    }
    q_point
}

fn encodeint(y: &BigInt, b: usize) -> Vec<u8> {
    let nbytes = b / 8;
    let (_sign, mut bytes) = y.to_bytes_le();
    bytes.resize(nbytes, 0);
    bytes.truncate(nbytes);
    bytes
}

fn encodepoint(p: &Vec<BigInt>, b: usize) -> Vec<u8> {
    let mut y_enc = encodeint(&p[1], b);
    let x_parity = (&p[0] & BigInt::one()).to_u8().unwrap();
    let last = y_enc.len() - 1;
    y_enc[last] |= x_parity << 7;
    y_enc
}

pub fn publickey(sk: &[u8], b: usize, q: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {
    let hval = h(sk);
    let mut a_bytes = hval[0..32].to_vec();

    a_bytes[0] &= 248;
    a_bytes[31] &= 127;
    a_bytes[31] |= 64;

    let a = BigInt::from_bytes_le(Sign::Plus, &a_bytes);
    let a_point = scalarmult(b_point, &a, q, d);
    encodepoint(&a_point, b)
}

fn hint(m: &[u8], _b: usize) -> BigInt {
    let hval = h(m);
    BigInt::from_bytes_le(Sign::Plus, &hval)
}

pub fn signature(
    m: &[u8],
    sk: &[u8],
    pk: &[u8],
    b: usize,
    q: &BigInt,
    l: &BigInt,
    d: &BigInt,
    b_point: &Vec<BigInt>,
) -> Vec<u8> {
    let hval = h(sk);
    let mut a_bytes = hval[0..32].to_vec();
    a_bytes[0] &= 248;
    a_bytes[31] &= 127;
    a_bytes[31] |= 64;

    let a = BigInt::from_bytes_le(Sign::Plus, &a_bytes);
    let prefix = &hval[32..64];

    let mut msg1 = Vec::new();
    msg1.extend_from_slice(prefix);
    msg1.extend_from_slice(m);

    let r = hint(&msg1, b).rem_euclid(l);
    let r_point = scalarmult(b_point, &r, q, d);
    let r_enc = encodepoint(&r_point, b);

    let mut msg2 = Vec::new();
    msg2.extend_from_slice(&r_enc);
    msg2.extend_from_slice(pk);
    msg2.extend_from_slice(m);

    let hram = hint(&msg2, b).rem_euclid(l);
    let s = (r + hram * a).rem_euclid(l);
    let s_enc = encodeint(&s, b);

    [r_enc, s_enc].concat()
}

fn isoncurve(p: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> bool {
    let x2 = (&p[0] * &p[0]).rem_euclid(q);
    let y2 = (&p[1] * &p[1]).rem_euclid(q);
    (x2 + y2 - BigInt::one() - d * &x2 * &y2).rem_euclid(q) == BigInt::zero()
}

fn decodeint(s: &[u8], b: usize) -> BigInt {
    BigInt::from_bytes_le(Sign::Plus, &s[..(b / 8)])
}

fn decodepoint(
    s: &[u8],
    b: usize,
    q: &BigInt,
    d: &BigInt,
    i_const: &BigInt,
) -> Result<Vec<BigInt>, &'static str> {
    let y = decodeint(s, b);
    let signbit = (s[b / 8 - 1] >> 7) & 1;
    let mut x = xrecover(&y, q, d, i_const);
    if (x.clone() & BigInt::one()).to_u8().unwrap() != signbit {
        x = q - x;
    }
    let p = vec![x, y];
    if !isoncurve(&p, q, d) {
        return Err("Point not on curve");
    }
    Ok(p)
}

pub fn checkvalid(
    s: &[u8],
    m: &[u8],
    pk: &[u8],
    b: usize,
    q: &BigInt,
    d: &BigInt,
    i_const: &BigInt,
    b_point: &Vec<BigInt>,
) -> bool {
    let r_enc = &s[..(b / 8)];
    let s_int = decodeint(&s[(b / 8)..], b);

    let a = match decodepoint(pk, b, q, d, i_const) {
        Ok(p) => p,
        Err(_) => return false,
    };
    let r_point = match decodepoint(r_enc, b, q, d, i_const) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let mut msg = Vec::new();
    msg.extend_from_slice(r_enc);
    msg.extend_from_slice(pk);
    msg.extend_from_slice(m);

    let hram = hint(&msg, b);
    let p1 = scalarmult(b_point, &s_int, q, d);
    let p2 = edwards(&r_point, &scalarmult(&a, &hram, q, d), q, d);

    p1[0] == p2[0] && p1[1] == p2[1]
}

