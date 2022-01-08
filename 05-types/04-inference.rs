// Rust's type inference engine looks at how a variable is used throughout a
// program to determine its type (not just at initialization).

fn main() {
    let elem = 5u8;

    // Here, the compiler doesn't know the exact type of `vec`. All it knows
    // is that it's vector (`Vec<_>`).
    let mut vec = Vec::new();

    vec.push(elem);

    println!("{:?}", vec);

    // Type annotations needed for `Vec<T>`.
    // Cannot infer type for type parameter `T`.
    // let mut vec2 = Vec::new();
    // println!("{:?}", vec2);
}
