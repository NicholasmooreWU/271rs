// src/main.rs
use f16_lib::F16; // adjust crate name appropriately if you named the crate differently
use f16_lib::{i32_to_f16, println_f16, add_f16s, sub_f16s, mul_f16s, div_f16s};

fn main() {
    let a = i32_to_f16(12);
    let b = i32_to_f16(123);
    let c = i32_to_f16(1234);

    println_f16(a); // prints like: 1.512*2^3 (approx)
    println_f16(b);
    println_f16(c);

    // arithmetic examples
    let sum = add_f16s(a, b);
    println!("sum: {}", f16_lib::format_f16(sum));

    let diff = sub_f16s(b, a);
    println!("diff: {}", f16_lib::format_f16(diff));

    let prod = mul_f16s(a, b);
    println!("prod: {}", f16_lib::format_f16(prod));

    let quot = div_f16s(b, a);
    println!("quot: {}", f16_lib::format_f16(quot));
}

