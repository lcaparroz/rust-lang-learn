enum VeryVerboseEnumOfThingsToDoWithNMumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNMumbers {
    fn run(&self, x: i32, y: i32,) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNMumbers;

fn main() {
    let x = Operations::Add;
    println!("Running operation 'x'... Result: {}", x.run(1, 2));

    let y = Operations::Subtract;
    println!("Running operation 'y'... Result: {}", y.run(2, 1));
}
