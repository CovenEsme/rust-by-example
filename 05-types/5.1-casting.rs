// Rust has no implicit type conversion (coercion) between primative types.

// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Cannot implicitly convert f32 to u8.
    // let integer: u8 = decimal;

    // Explicit conversions.
    let integer = decimal as u8;
    let character = integer as char;

    // There are limitations in conversion rules - float can't be directly
    // converted to a char.
    // error[E0604]: only `u8` can be cast as `char`, not `f32`.
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // When casting any value to an unsigned type, T, T::MAX + 1 is added or
    // subtracted until the value fits into the new type.

    // 1000 already fits in a u16.
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232 (keeps first 8 LSBs and truncates MSB).
    println!("1000 as a u8 is: {}", 1000 as u8);

    // For positive numbers, this is the same as modulus.
    println!("1000 mod 256 is: {}", 1000 % 256);

    // -1 + 256 = 255
    println!("-1 as a u8 is: {}", (-1i8) as u8);

    // When casting an unsigned type to a signed type, the (bitwise) result is
    // the same as first casting to the corresponding unsigned type. Then, if
    // the MSB is a 1, the value is negative (two's complement).
    // E.g. 128u16 -> i8 = 128u8 -> 128i8.
    println!("128 as a i8 is: {}", 128 as i8);

    // Unless it already fits.
    println!("128 as a i16 is: {}", 128 as i16);

    // With the first example.
    println!("1000 as a u8 is: {}", 1000 as u8);
    println!("232 as a i8 is: {}", 232 as i8);
    println!("1000 as a i8 is: {}", 1000 as i8);

    // The `as` keyword performs a *saturating cast* when casting from a float
    // to an int. If the float is larger than the upper bound or smaller than
    // the lower bound, the returned value will be equal to the bound crossed.
    println!("300.0 as a u8 is: {}", 300.0_f32 as u8);
    println!("-100.0 as a u8 is: {}", -100.0_f32 as u8);
    println!("nan as u8 is: {}", f32::NAN as u8);

    // This behaviour incurs a small runtime cost and can be avoided with
    // unsafe methods, however the results might overflow and return
    // **unsound values**. Use these methods wisely.
    unsafe {
        println!("Using unsafe .to_int_unchecked::<u8>().");
        println!("300.0 as a u8 is: {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as a u8 is: {}",
                    (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as a u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
