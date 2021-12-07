// `use` is rust's way of importing code.
use std::fmt;

// Defines a structure that `fmt::Display` will be implemented for.
// Structure(i32) is a tuple containing an i32.
struct Structure(i32);

// To use the `{}` marker when printing, the trait `fmt::Display` needs to be
// implemented.
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// The trait fmt::Display needs the exact signature used above
// fmt(&self, f: &mut fmt::Formatter) -> fmt::Result

// The function writes strictly the first element into the given output
// stream: `f`. It returns `fmt::Result` which indicates if the operation
// succeeded or failed.

// `write!` uses similar syntax to `println!`.


// Because there is no single ideal style for all types, the `std` library
// doesn't dictate one. Therefore, `fmt::Display` isn't implemented for any of
// the generic containers (fmt::Debug still can be though).

// However, `fmt::Display` can be implemented for containers that aren't
// generic.

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}


#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}


fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug {:?}", minmax);

    let big_range   = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small range is {small}",
             small = small_range,
             big   = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);


    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);


    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
