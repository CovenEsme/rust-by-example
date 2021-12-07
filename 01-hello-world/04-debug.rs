// Cannot be printed with fmt::Display or fmt::Debug.
// Compiler shows a warning as UnPrintable cannot be constructed.
struct UnPrintable(i32);

// The derive attribute creates the implementation to make the struct
// printable with fmt::Debug.
#[derive(Debug)]
struct DebugPrintable(i32);



#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // "Now Structure(3) will print!"
    println!("Now {:?} will print!", Structure(3));

    // "Now Deep(Structure(7)) will print!"
    println!("Now {:?} will print!", Deep(Structure(7)));


    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print.
    println!("{:#?}", peter);
}
