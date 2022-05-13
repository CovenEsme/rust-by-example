// The `From` trait is used to define how a type can be created from other
// types. e.g. converting from a `str` to a `String`.
// let my_str = "hello";
// let my_string = String::from(my_str);

// The `Into` trait is the reciprocal of the `From` trait. If a type has the
// `From` trait implemented, `Into` will call it when necessary.

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    // Using `from`.
    let num = Number::from(30);
    println!("My number is {:?}", num);

    // Using `into`.
    let int = 5;
    let num2: Number = int.into();
    println!("My number is {:?}", num2);
}
