use num::*;
use num_bigint::BigInt;
use num_traits::Euclid;

fn main() {
    const B: usize = 256;
    
    let q = BigInt::from(2).pow(255) - BigInt::from(19);

    let l_suffix = BigInt::parse_bytes(b"27742317777372353535851937790883648493", 10).unwrap();
    let l = BigInt::from(2).pow(252) + l_suffix;

    let n_d = BigInt::from(-121665);
    let d_inv = inv(&BigInt::from(121666), &q);
    let d = (n_d * d_inv).rem_euclid(&q);

    let exponent_i = (&q - BigInt::from(1)) / BigInt::from(4);
    let i_const = expmod(&BigInt::from(2), &exponent_i, &q);
    
    let by = (BigInt::from(4) * inv(&BigInt::from(5), &q)).rem_euclid(&q);
    
    let bx = xrecover(&by, &q, &d, &i_const);
    
    let b_point: Vec<BigInt> = vec![bx.rem_euclid(&q), by.rem_euclid(&q)];

    let sk = vec![
        0x4c, 0xcd, 0x08, 0x9b, 0x28, 0xff, 0x96, 0xda, 0x9d, 0xb6, 0xc3, 0x46, 0xec, 0x11, 0x4e, 0x0f,
        0x5b, 0x8a, 0x31, 0x9f, 0x35, 0xab, 0xa6, 0x24, 0xda, 0x8c, 0xf6, 0xed, 0x4f, 0xb8, 0xa6, 0xfb,
        0x3d, 0x40, 0x17, 0xc3, 0xe8, 0x43, 0x89, 0x5a, 0x92, 0xb7, 0x0a, 0xa7, 0x4d, 0x1b, 0x7e, 0xbc,
        0x9c, 0x98, 0x2c, 0xcf, 0x2e, 0xc4, 0x96, 0x8c, 0xc0, 0xcd, 0x55, 0xf1, 0x2a, 0xf4, 0x66, 0x0c
    ];

    let m = b"72";

    let pk = publickey(&sk, B, &q, &d, &b_point);

    let sig = signature(m, &sk, &pk, B, &q, &l, &d, &b_point);
    
    dbg!(checkvalid(&sig, m, &pk, B, &q, &d, &i_const, &b_point));
}

