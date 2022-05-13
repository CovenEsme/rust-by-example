// The `type` keyword can be used to give a new name to existing types. They
// must have UpperCamelCase names to avoid compiler warnings.

// The main use of aliases is to reduce boilerplate.
// e.g. the `IoResult<T>` type is an alias for the `Result<T, IoError>` type.

type NanoSecond = u64;
type Inch = u64;

// Silences the case warning with an attribute.
#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // Type aliases *don't* provide any extra type safety as aliases
    // aren't new types.
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}
