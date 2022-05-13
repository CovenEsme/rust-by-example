// There are two types of constants:
// * `const`: an unchangeable value,
// * `static`: possibly `mut`able with a `static` lifetime.
//
// Accessing or modifying a static variable is `unsafe`.

static LANGUAGE: &str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}
