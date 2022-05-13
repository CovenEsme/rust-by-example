// Create an `enum` to classify a web event. Note how both names and type
// information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // Unit-like structs.
    PageLoad,
    PageUnload,

    // Tuple structs.
    KeyPress(char),
    Paste(String),

    // C-like structs
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad   => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),

        // Destructure `c` from inside the enum.
        WebEvent::KeyPress(c) => println!("Pressed: '{}'", c),
        WebEvent::Paste(s)    => println!("Pasted:  \"{}\"", s),

        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("Clicked at x={}, y={}", x, y);
        },
    }
}


#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Type alias - so you don't need to use the long name.
// Commonly used in `impl` blocks with the `Self` alias.
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add      => x + y,
            Self::Subtract => x - y,
        }
    }
}


fn main() {
    let pressed = WebEvent::KeyPress('x');

    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("My text".to_owned()) ;
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;
    println!("{:?}", x);
}
