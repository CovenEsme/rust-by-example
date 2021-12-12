// There are 3 types of structures created with the `struct` keyword:
// * Tuple structs: named tuples,
// * C structs: composite, mixed-type data stored in contiguous memory,
// * Unit structs: field-less structures; useful for generics.

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct.
struct Unit;

// A tuple struct.
struct Pair(i32, f32);

// A struct with two fields.
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields in other structs.
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left:  Point,
    bot_right: Point,
}

fn rect_area(rect: Rectangle) {
    let Rectangle { top_left:  Point { x: top_left_x,  y: top_left_y },
                    bot_right: Point { x: bot_right_x, y: bot_right_y }
                  } = rect;

    let rect_len   = (top_left_y - bot_right_y).abs();
    let rect_width = (top_left_x - bot_right_x).abs();

    println!("Rectangle area: {}", rect_len*rect_width);
}

fn square(point: Point, side_len: f32) -> Rectangle {
    let top_left_point  = Point { x: point.x, y: point.y + side_len };
    let bot_right_point = Point { x: point.x + side_len, y: point.y };

    let rect = Rectangle { top_left: top_left_point,
                           bot_right: bot_right_point };

    rect
}

fn main() {
    // Create struct with field init shorthand.
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("Point coords: ({}, {})", point.x, point.y);

    // Make a new point using the struct update syntax to use the y value from
    // the already defined point.
    let bot_right = Point { x: 5.2, ..point };
    println!("Second point: ({}, {})", bot_right.x, bot_right.y);

    // Destructure the point using a `let` binding.
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // Struct instantiation is an expression too.
        top_left:  Point { x: left_edge,       y: top_edge },
        bot_right: Point { x: left_edge - 7.0, y: top_edge - 4.0 },
    };

    // Instantiate a unit struct.
    let _unit = Unit;

    // Instantiate a tuple struct.
    let pair = Pair(1, 0.1);
    println!("Pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct.
    let Pair(integer, decimal) = pair;
    println!("Pair contains {:?} and {:?}", integer, decimal);

    println!("{:?}", _rectangle);
    rect_area(_rectangle);

    let square_rect: Rectangle = square(point, 5.0);
    println!("{:?}", square_rect);
    rect_area(square_rect);
}
