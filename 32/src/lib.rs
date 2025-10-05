//! Library crate exporting three macros:
//! - choice!(e, f, g)  -> (e & f) ^ ((!e) & g)
//! - median!(e, f, g)  -> (e & f) | (e & g) | (f & g)
//! - rotate!(x, n)     -> right rotate x by n (n may be >= bit-width)

/// Bitwise "choice" ternary: for each bit, choose bit from `f` if `e` bit is 1, else from `g`.
#[macro_export]
macro_rules! choice {
    ( $e:expr, $f:expr, $g:expr ) => {
        {
            // evaluate each expression once
            let e_val = $e;
            let f_val = $f;
            let g_val = $g;
            // bitwise choice: (e & f) ^ ((!e) & g)
            (e_val & f_val) ^ ((!e_val) & g_val)
        }
    };
}

/// Bitwise "median" / majority: for each bit, majority of the three bits.
/// Equivalent to: (e & f) | (e & g) | (f & g)
#[macro_export]
macro_rules! median {
    ( $e:expr, $f:expr, $g:expr ) => {
        {
            let e_val = $e;
            let f_val = $f;
            let g_val = $g;
            (e_val & f_val) | (e_val & g_val) | (f_val & g_val)
        }
    };
}

/// Right rotate macro: rotates value `x` right by `n` bits.
/// This uses the integer `rotate_right` method and guards against shifts >= bit-width
/// by reducing n modulo the bit width determined at runtime.
#[macro_export]
macro_rules! rotate {
    ( $x:expr, $n:expr ) => {
        {
            let x_val = $x;
            // compute bit width of the value
            let bits = (std::mem::size_of_val(&x_val) * 8) as u32;
            let shift = ($n as u32) % bits;
            // use rotate_right method available on integer types (u8,u16,u32,u64,usize,...)
            x_val.rotate_right(shift)
        }
    };
}

