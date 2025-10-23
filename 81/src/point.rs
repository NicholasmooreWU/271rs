// src/point.rs
use num_bigint::BigUint;

/// A point on the curve (x, y), both BigUint modulo q
#[derive(Debug, Clone)]
pub struct Point {
    pub x: BigUint,
    pub y: BigUint,
}

impl Point {
    pub fn new(x: BigUint, y: BigUint) -> Self {
        Point { x, y }
    }
}

/// Edwards addition (placeholder)
pub fn edwards(_p: &Point, _q: &Point) -> Point {
    unimplemented!("implement point addition per the Python reference");
}

/// Scalar multiplication (double-and-add as in Python)
pub fn scalarmult(_p: &Point, _e: &BigUint) -> Point {
    unimplemented!("implement scalarmult");
}

