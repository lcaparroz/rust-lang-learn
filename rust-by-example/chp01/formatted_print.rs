fn main() {
    println!("{} days", 31);

    // Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    // Special formatting (after ':')
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // Text alignment
    println!("{number:>width$}", number=1, width=6);

    // Number padding
    println!("{number:>0width$}", number=1, width=6);

    // Number of arguments checking at compile time
    // println!("My name is {0}, {1} {0}", "Bond"); // This won't compile
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Custom types which don't implement a method for displaying can't be "stringfied"
    #[allow(dead_code)]
    struct Structure(i32);

    // println!("This struct `{}` won't print...", Structure(3));

    // Decimal precision
    let pi = 3.141592;
    println!("{0:.1$}", pi, 3)
}
