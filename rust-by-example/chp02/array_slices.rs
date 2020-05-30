use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice is: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the arrys: {}", xs[1]);
    println!("array size: {}", xs.len());

    println!("array xs occupies {} bytes", mem::size_of_val(&xs));
    println!("array ys occupies {} bytes", mem::size_of_val(&ys));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Out of bounds error
    // println!("{}", xs[5]);
}
