// Scalar types:
//
// Signed integers:      i8, i16, i32, i64, i128, isize (pointer size).
// Unsigned integers:    u8, u16, u32, u64, u128, usize (pointer size).
// Floating point:       f32, f64.
// Characters (4 bytes): char (Unicode scalar values like `a`, `α` and `∞`.
// Boolean:              bool (either `true` or `false`).
// Unit type:            () (only possible value is an empty tuple).
//
// Compound types:
// Arrays: [1, 2, 3]
// Tuples: (1, true)

fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation.
    let an_integer   = 5i32; // Suffix annotation.

    let default_float   = 3.0; // Defaults to f64.
    let default_integer = 7;   // Defaults to i32.

    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // A mutable i32.
    mutable = 21;

    // A mutable variable's type cannot be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;
}
