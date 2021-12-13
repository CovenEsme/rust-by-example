// Variable bindings are constrained to blocks. Blocks are enclosed by {}.

fn main() {
    let long_lived_binding = 1;

    // This block has a smaller scope than the main function.
    {
        let short_lived_binding = 2;

        println!("Inner short: {}", short_lived_binding);
    }

    // Cannot find value in this scope.
    // println!("Outer short: {}", short_lived_binding);

    println!("Outer long: {}", long_lived_binding);

    // Variable shadowing is allowed.
    let shadowed_binding = 1;

    {
        println!("Before being shadowed: {}", shadowed_binding);

        let shadowed_binding = "abc";

        println!("Shadowed in inner block: {}", shadowed_binding);
    }

    println!("Outside inner block: {}", shadowed_binding);

    // This binding shadows the previous binding.
    let shadowed_binding = 2;

    println!("Shadowed in outer block: {}", shadowed_binding);
}
