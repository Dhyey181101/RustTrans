

use std::ops::Add;

const INPUT_INVALID: &'static str = "Input is invalid, crash gracefully";

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn add_points(p: Point, r: Point) -> Point {
    p + r
}

fn main() {
    let p1 = Point { x: -4.057333716302652e-152, y: 8.442542515363439e-227 };
    let p2 = Point { x: -3.129951052382102e+105, y: 4.519902841294e-312 };
    println!("{:?}", add_points(p1, p2));

    let p3 = Point { x: -1.622933486521061e-151, y: -1.5274681817499434e-151 };
    let p4 = Point { x: 7.449719389772673e-309, y: -1.5873679807699466e-151 };
    println!("{:?}", add_points(p3, p4));

    let p5 = Point { x: 3.89585165e-315, y: -8.98846567431158e+307 };
    let p6 = Point { x: 5.5075959114172704e-157, y: -9.08282700831338e+307 };
    // println!("{:?}", add_points(p5, p6)); // Uncommenting this line will cause a panic.

    let p7 = Point { x: 3.48139935e-315, y: -8.98846567431158e+307 };
    let p8 = Point { x: -9.935025383384232e+235, y: -8.988465674316845e+307 };
    // println!("{:?}", add_points(p7, p8)); // Uncommenting this line will cause a panic.
}

