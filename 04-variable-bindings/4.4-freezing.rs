// When data is bound by the same name immutability, it also freezes. Frozen
// data can't be modified until the immutable binding goes out of scope.

fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`.
        let _mutable_integer = _mutable_integer;

        // Cannot assign twice to immutable variable.
        // _mutable_integer = 50;
    }

    _mutable_integer = 3;
}
