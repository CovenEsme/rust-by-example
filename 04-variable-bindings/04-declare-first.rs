// You *can* declare variable bindings first and init them later (like in java)
// but it's rarely used and may lead to uninitialized variables.

fn main() {
    let a_binding;

    {
        let x = 2;

        a_binding = x * x;

        println!("A binding: {}", a_binding);
    }

    let another_binding;

    // Borrow of possibly-uninitialized variable.
    // println!("Another binding: {}", another_binding);

    another_binding = 1;

    println!("Another binding: {}", another_binding);
}
