// Variable bindings can be type annotated but in most cases the compiler will
// be able to infer the type from context.

fn main() {
    let an_integer = 1u32;
    let a_boolean  = true;
    let unit       = ();

    // Copy an_integer into copied_integer.
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler gives warnings about unused variables; these warnings can
    // be silenced by prefixing a variable name with an underscore.
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
}
