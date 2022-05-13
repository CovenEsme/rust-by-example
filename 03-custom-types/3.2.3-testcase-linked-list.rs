// Enums are commonly used to create linked-lists.
use crate::List::*;

enum List {
    // A tuple with an element and a pointer to the next node.
    Cons(u32, Box<List>),
    Nil, // Signifies the end of a linked-list.
}

// Methods can be attached to enums.
impl List {
    // Create an empty list.
    // `Nil` has type `List`.
    fn new() -> List { Nil }

    // Consume a list and return the same list with a new element at the start.
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type `List`.
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        // `self` has type `&List`; `&self` has type `List`.
        // `match`ing on a 'concrete' type `T` is preferred to references `&T`.
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create an empty linked-list.
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("Linked-list has length: {}", list.len());
    println!("{}", list.stringify());
}
