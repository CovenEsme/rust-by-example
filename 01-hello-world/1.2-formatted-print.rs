// Formatted text is handled by macros in std::fmt.
// This includes:
// format! for *just* formatting strings.
// print! and println! for displaying formatted text to stdout.
// eprint! and eprintln! for displaying formatted text to stderr.

fn main() {
    // Like python, {} gets replaced with the value of the argument.
    println!("{} days", 31);

    // A suffix can be used to determine what type 32 is. i32 is the default.

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Arguments can have arbitrary names.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting after :.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // Number padding.
    println!("{number:width$}", number=1, width=6);

    // Zero padding.
    println!("{number:0>width$}", number=1, width=6);

    // Removing "James" as an argument causes a compile time error.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Create a structure named "Structure" which contains an i32.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));

    // Activities:
    println!("Pi is roughly {pi:.3}", pi=3.141592);
}
