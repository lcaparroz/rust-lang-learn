fn main() {
    let logical: bool = true;

    // Regular vs suffix annotation
    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    // Type inference
    let default_float = 3.0;
    let default_integer = 7;

    // mutable variables
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;

    // Can't change variable type
    // mutable = true;

    // Variable shadowing
    let mutable = true;
}
