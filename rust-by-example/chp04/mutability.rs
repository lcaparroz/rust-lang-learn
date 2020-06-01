fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // A variable can't be assigned twice, unless declared with `mut`
    // _immutable_binding += 1;
}
