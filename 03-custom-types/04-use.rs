// `use` can be used (heh) so manual scoping isn't necessary.

#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are available without manual scoping.
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor; // equivalent to `Status::Poor`.
    let work   = Civilian;

    match status {
        // No need for scoping because of the `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // vs explicitly scoping.
        Work::Civilian => println!("Civilians work!"),
        Work::Soldier  => println!("Soldiers fight!"),
    }
}
