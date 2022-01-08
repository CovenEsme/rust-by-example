fn main() {
    // Suffixed literals.
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals - their types depend on how they are used.
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes.
    // `std::mem::size_of_val` is a function (`size_of_val`) defined in a
    // module (`mem`) which is defined in a `crate` (`std`).
    println!("Size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("Size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("Size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("Size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("Size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
