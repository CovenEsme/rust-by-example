// Try `write!` to see if it errors. If it errors, return
// the error. Otherwise continue.
// write!(f, "{}", value)?;

use std::fmt;

// Defines a structure containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing and create a ref to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{count}: {value}", count=count, value=v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
