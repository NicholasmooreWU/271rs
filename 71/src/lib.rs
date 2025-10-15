#![allow(non_camel_case_types)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ix {
    pub sign: bool,     // false = positive or zero, true = negative
    pub vals: Vec<u64>, // little-endian limbs: vals[0] = least significant
}

fn normalize_vals(v: &mut Vec<u64>) {
    while let Some(&last) = v.last() {
        if last == 0 {
            v.pop();
        } else {
            break;
        }
    }
}

pub fn zero_ix() -> ix {
    ix {
        sign: false,
        vals: vec![],
    }
}

pub fn u64_to_ix(val: u64) -> ix {
    if val == 0 {
        zero_ix()
    } else {
        ix {
            sign: false,
            vals: vec![val],
        }
    }
}

// Compare magnitudes: return true if a >= b
pub fn gte_mag(a_vals: &Vec<u64>, b_vals: &Vec<u64>) -> bool {
    if a_vals.len() != b_vals.len() {
        return a_vals.len() > b_vals.len();
    }
    for i in (0..a_vals.len()).rev() {
        if a_vals[i] != b_vals[i] {
            return a_vals[i] > b_vals[i];
        }
    }
    true
}

// Compare magnitudes: returns -1 if a<b, 0 if equal, 1 if a>b
pub fn cmp_mag(a_vals: &Vec<u64>, b_vals: &Vec<u64>) -> i8 {
    if a_vals.len() != b_vals.len() {
        return if a_vals.len() < b_vals.len() { -1 } else { 1 };
    }
    for i in (0..a_vals.len()).rev() {
        if a_vals[i] < b_vals[i] {
            return -1;
        }
        if a_vals[i] > b_vals[i] {
            return 1;
        }
    }
    0
}

// Add magnitudes: little-endian vectors
pub fn add_mag(aug_vals: &Vec<u64>, add_vals: &Vec<u64>) -> Vec<u64> {
    let mut out = Vec::with_capacity(std::cmp::max(aug_vals.len(), add_vals.len()) + 1);
    let mut carry: u128 = 0;
    let n = std::cmp::max(aug_vals.len(), add_vals.len());
    for i in 0..n {
        let a = if i < aug_vals.len() {
            aug_vals[i] as u128
        } else {
            0
        };
        let b = if i < add_vals.len() {
            add_vals[i] as u128
        } else {
            0
        };
        let sum = a + b + carry;
        out.push((sum & ((1u128 << 64) - 1)) as u64);
        carry = sum >> 64;
    }
    if carry != 0 {
        out.push(carry as u64);
    }
    normalize_vals(&mut out);
    out
}

// Subtract magnitudes: minuend - subtrahend; precondition: minuend >= subtrahend
pub fn sub_mag(min_vals: &Vec<u64>, sub_vals: &Vec<u64>) -> Vec<u64> {
    let mut out = Vec::with_capacity(min_vals.len());
    let mut borrow: i128 = 0;
    for i in 0..min_vals.len() {
        let a = min_vals[i] as i128;
        let b = if i < sub_vals.len() {
            sub_vals[i] as i128
        } else {
            0
        };
        let mut cur = a - b - borrow;
        if cur < 0 {
            cur += 1i128 << 64;
            borrow = 1;
        } else {
            borrow = 0;
        }
        out.push(cur as u64);
    }
    normalize_vals(&mut out);
    out
}

// Sign-aware addition
pub fn add_ix(a: &ix, b: &ix) -> ix {
    if a.vals.is_empty() {
        return b.clone();
    }
    if b.vals.is_empty() {
        return a.clone();
    }

    if a.sign == b.sign {
        let vals = add_mag(&a.vals, &b.vals);
        let mut res = ix { sign: a.sign, vals };
        normalize_vals(&mut res.vals);
        if res.vals.is_empty() {
            res.sign = false;
        }
        res
    } else {
        if gte_mag(&a.vals, &b.vals) {
            let vals = sub_mag(&a.vals, &b.vals);
            let mut res = ix { sign: a.sign, vals };
            normalize_vals(&mut res.vals);
            if res.vals.is_empty() {
                res.sign = false;
            }
            res
        } else {
            let vals = sub_mag(&b.vals, &a.vals);
            let mut res = ix { sign: b.sign, vals };
            normalize_vals(&mut res.vals);
            if res.vals.is_empty() {
                res.sign = false;
            }
            res
        }
    }
}

// Sign-aware subtraction
pub fn sub_ix(a: &ix, b: &ix) -> ix {
    let b = ix {
        sign: !b.sign,
        vals: b.vals.clone(),
    };
    add_ix(a, &b)
}

// Multiplication (schoolbook method)
pub fn mul_ix(a: &ix, b: &ix) -> ix {
    if a.vals.is_empty() || b.vals.is_empty() {
        return zero_ix();
    }
    let n = a.vals.len();
    let m = b.vals.len();
    let mut out = vec![0u64; n + m];
    for i in 0..n {
        let mut carry: u128 = 0;
        for j in 0..m {
            let idx = i + j;
            let prod = (a.vals[i] as u128) * (b.vals[j] as u128) + (out[idx] as u128) + carry;
            out[idx] = (prod & ((1u128 << 64) - 1)) as u64;
            carry = prod >> 64;
        }
        if carry != 0 {
            // add carry to next limb (handle possible carry chain)
            let mut k = i + m;
            let mut c = carry as u128;
            while c != 0 {
                if k >= out.len() {
                    out.push(0);
                }
                let sum = (out[k] as u128) + c;
                out[k] = (sum & ((1u128 << 64) - 1)) as u64;
                c = sum >> 64;
                k += 1;
            }
        }
    }
    normalize_vals(&mut out);
    let sign = a.sign ^ b.sign;
    let mut res = ix { sign, vals: out };
    if res.vals.is_empty() {
        res.sign = false;
    }
    res
}

// ----------------- Division helpers -----------------

// bit length of magnitude (0 => 0)
pub fn bit_len(vals: &Vec<u64>) -> usize {
    if vals.is_empty() {
        return 0;
    }
    let top = vals.last().unwrap();
    let top_bits = 64 - top.leading_zeros() as usize;
    (vals.len() - 1) * 64 + top_bits
}

// left shift magnitude by k bits
pub fn shl_mag(vals: &Vec<u64>, k: usize) -> Vec<u64> {
    if vals.is_empty() {
        return vec![];
    }
    let limb_shift = k / 64;
    let rem = k % 64;
    let mut out = vec![0u64; limb_shift + vals.len() + 1];
    if rem == 0 {
        for i in 0..vals.len() {
            out[i + limb_shift] = vals[i];
        }
    } else {
        for i in 0..vals.len() {
            let low = vals[i].wrapping_shl(rem as u32);
            let high = vals[i].wrapping_shr((64 - rem) as u32);
            out[i + limb_shift] |= low;
            out[i + limb_shift + 1] |= high;
        }
    }
    normalize_vals(&mut out);
    out
}

// add a single bit (1 << k) into vals (little-endian) in-place
pub fn add_bit_to_vec(vals: &mut Vec<u64>, k: usize) {
    let limb = k / 64;
    let bit = k % 64;
    if vals.len() <= limb {
        vals.resize(limb + 1, 0);
    }
    let mut carry: u128 = (1u128 << bit) as u128;
    let mut i = limb;
    while carry != 0 {
        if i >= vals.len() {
            vals.push(0);
        }
        let sum = (vals[i] as u128) + carry;
        vals[i] = (sum & ((1u128 << 64) - 1)) as u64;
        carry = sum >> 64;
        i += 1;
    }
}

// div_rem on magnitudes: returns (q_vals, r_vals)
pub fn div_rem_mag(a_vals: &Vec<u64>, b_vals: &Vec<u64>) -> (Vec<u64>, Vec<u64>) {
    if b_vals.is_empty() {
        panic!("division by zero");
    }
    if a_vals.is_empty() {
        return (vec![], vec![]);
    }
    if cmp_mag(a_vals, b_vals) < 0 {
        return (vec![], a_vals.clone());
    }

    let mut rem = a_vals.clone();
    let mut q: Vec<u64> = vec![];

    let a_bits = bit_len(a_vals);
    let b_bits = bit_len(b_vals);
    let mut shift = a_bits as isize - b_bits as isize;

    while shift >= 0 {
        let shift_usize = shift as usize;
        let t = shl_mag(b_vals, shift_usize);
        if cmp_mag(&t, &rem) <= 0 {
            rem = sub_mag(&rem, &t);
            add_bit_to_vec(&mut q, shift_usize);
        }
        shift -= 1;
    }
    normalize_vals(&mut q);
    normalize_vals(&mut rem);
    (q, rem)
}

// Division returning quotient ix
pub fn div_ix(a: &ix, b: &ix) -> ix {
    if b.vals.is_empty() {
        panic!("division by zero");
    }
    if a.vals.is_empty() {
        return zero_ix();
    }
    let (q_vals, _) = div_rem_mag(&a.vals, &b.vals);
    let mut q = ix {
        sign: a.sign ^ b.sign,
        vals: q_vals,
    };
    normalize_vals(&mut q.vals);
    if q.vals.is_empty() {
        q.sign = false;
    }
    q
}

// Remainder (a mod b), sign same as dividend (a)
pub fn rem_ix(a: &ix, b: &ix) -> ix {
    if b.vals.is_empty() {
        panic!("division by zero");
    }
    if a.vals.is_empty() {
        return zero_ix();
    }
    let (_, r_vals) = div_rem_mag(&a.vals, &b.vals);
    let mut r = ix {
        sign: a.sign,
        vals: r_vals,
    };
    normalize_vals(&mut r.vals);
    if r.vals.is_empty() {
        r.sign = false;
    }
    r
}

// ----------------- I/O helpers -----------------

// Parse hex string (0x...) to ix
pub fn h2i_ix(s: &str) -> ix {
    let s = s.trim();
    let s = s.strip_prefix("0x").unwrap_or(s);
    if s.is_empty() {
        return zero_ix();
    }
    let mut vals = vec![];
    let mut i = s.len();
    while i > 0 {
        let start = if i >= 16 { i - 16 } else { 0 };
        let chunk = &s[start..i];
        let limb = u64::from_str_radix(chunk, 16).expect("bad hex input");
        vals.push(limb);
        i = start;
    }
    normalize_vals(&mut vals);
    ix { sign: false, vals }
}

// Print ix as hex string (no 0x prefix, with sign if negative)
pub fn see_ix(x: &ix) {
    if x.vals.is_empty() {
        print!("0");
        return;
    }
    if x.sign {
        print!("-");
    }
    let mut parts = vec![];
    for &limb in x.vals.iter().rev() {
        parts.push(format!("{:016x}", limb));
    }
    let mut joined = parts.join("");
    while joined.len() > 1 && joined.starts_with('0') {
        joined.remove(0);
    }
    print!("{}", joined);
}
