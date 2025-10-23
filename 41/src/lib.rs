// src/lib.rs
use num_bigint::BigUint;
use num_traits::One;

/// Exercise 0: simple trial-division is_prime (fine for these small primes)
pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n % 2 == 0 { return n == 2; }
    let mut d = 3u64;
    while d.saturating_mul(d) <= n {
        if n % d == 0 { return false; }
        d += 2;
    }
    true
}

/// Exercise 1: approximate fractional bits using f64 (demonstrates insufficient precision)
pub fn approx_frac_using_f64(p: u64) -> u64 {
    let s = (p as f64).sqrt();
    let frac = s - s.floor();
    let scaled = (frac * (2.0f64.powi(64))).floor();
    scaled as u64
}

/// Exercise 3: exact floor(frac(sqrt(p)) * 2^64)
/// Uses big integers and binary search to find the largest f in [0,2^64-1]
/// s.t. (floor(sqrt(p))<<64 + f)^2 <= p * 2^128
pub fn compute_frac_bits_sqrt(p: u64) -> u64 {
    // floor(sqrt(p)) - safe using f64 for small p
    let floor_sqrt_p = (p as f64).sqrt().floor() as u64;

    // target = p * 2^128
    let target = BigUint::from(p) << 128;

    // base = floor_sqrt_p << 64
    let base = BigUint::from(floor_sqrt_p) << 64;

    // binary search for f in [0, 2^64-1]
    let mut lo: u128 = 0;
    let mut hi: u128 = (1u128 << 64) - 1;

    while lo < hi {
        // upper-mid to avoid infinite loop
        let mid = lo + ((hi - lo + 1) / 2);
        let candidate = &base + BigUint::from(mid as u64);
        let candidate_sq = &candidate * &candidate;
        if candidate_sq <= target {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }

    lo as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    // expected constants: floor(frac(sqrt(p)) * 2^64)
    const EXPECTED: [u64; 8] = [
        0x6a09e667f3bcc908u64, // p=2
        0xbb67ae8584caa73bu64, // p=3
        0x3c6ef372fe94f82bu64, // p=5
        0xa54ff53a5f1d36f1u64, // p=7
        0x510e527fade682d1u64, // p=11
        0x9b05688c2b3e6c1fu64, // p=13
        0x1f83d9abfb41bd6bu64, // p=17
        0x5be0cd19137e2179u64, // p=19
    ];

    #[test]
    fn test_is_prime_small() {
        let primes = [2u64,3,5,7,11,13,17,19];
        for &p in primes.iter() {
            assert!(is_prime(p), "expected {} to be prime", p);
        }
        for &c in [0u64,1,4,6,8,9,10,12,14,15,16,18].iter() {
            assert!(!is_prime(c), "expected {} to be composite", c);
        }
    }

    #[test]
    fn test_compute_constants_match_expected() {
        let primes = [2u64,3,5,7,11,13,17,19];
        for (i, &p) in primes.iter().enumerate() {
            let got = compute_frac_bits_sqrt(p);
            assert_eq!(got, EXPECTED[i], "mismatch for p={}", p);
        }
    }

    #[test]
    fn approx_is_insufficient() {
        // approx may differ in low bits — this test asserts it is not equal
        let primes = [2u64,3,5,7,11,13,17,19];
        let mut different = false;
        for &p in primes.iter() {
            let a = approx_frac_using_f64(p);
            let exact = compute_frac_bits_sqrt(p);
            if a != exact {
                different = true;
                break;
            }
        }
        assert!(different, "expected approx using f64 to differ from exact for at least one prime");
    }
}

