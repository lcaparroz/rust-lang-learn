#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rectangle;

    let x_side_size = (x2 - x1).abs();
    let y_side_size = (y2 - y1).abs();

    x_side_size * y_side_size
}

fn square(point: Point, side_size: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            y: point.y + side_size,
            ..point
        },
        bottom_right: Point {
            x: point.x + side_size,
            ..point
        },
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: top_edge, y: left_edge } = point;
    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let rectangle = Rectangle {
        top_left: Point { x: 1.0, y: 5.0 },
        bottom_right: Point { x: 5.0, y: 1.0 },
    };

    println!("Given the rectangle: {:?}", rectangle);
    println!("Its area is: {}", rect_area(rectangle));

    let new_square = square(Point { x: 1.0, y: 1.0 }, 4.0);
    println!("A square built from a point: {:?}", new_square);
}
