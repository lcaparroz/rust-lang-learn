fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // The last expression evaluated in a block define the block expression value
        x_cube + x_squared + x
    };

    // A semicolon in the last expression of a block defines the block expression as `()`
    let z = {
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
