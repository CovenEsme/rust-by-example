// Arrays: a collection of same type objects with known length at complie time.
// Slices: similar to arrays but with unknown length at compile time.

use std::mem;

// Borrowing a slice.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements.", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5]; // same as `let xs = [1, 2, 3, 4, 5]`.

    // Initializes all elements to the same value (0).
    let ys: [i32; 500] = [0; 500];

    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form `[starting_index..ending_index]`.
    // `starting_index` is the first position of the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // Out of bound indexing causes a compile time error.
    // println!("{}", xs[5]);
}
