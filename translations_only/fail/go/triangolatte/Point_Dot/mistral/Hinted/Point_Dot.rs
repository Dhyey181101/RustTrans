
use std::option::Option;
use std::boxed::Box;

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn dot(p: Point, r: Point) -> f64 {
    p.x * r.x + p.y * r.y
}

fn main() {
    let p = Point { x: -5.486129281209763e+303, y: 1.0091441337824653e-209 };
    let r = Point { x: 2.1222213645e-314, y: 5.9055142862564e-311 };
    println!("{}", dot(p, r));

    let p = Point { x: 1.6259760885483456e-260, y: -7.896958926786568e-84 };
    let r = Point { x: 1.447950069e-314, y: 0.0 };
    println!("{}", dot(p, r));

    let p = Point { x: -9.25649984299006e+303, y: -7.91405437059041e+269 };
    let r = Point { x: -7.914059250736398e+269, y: 4.243731533e-314 };
    match dot(p, r) {
        _ => println!("None"),
    }

    let p = Point { x: -9.655585032969467e+305, y: 9.096284213515174e-309 };
    let r = Point { x: 4.460153309817341e+43, y: 2.0522675061496073e-289 };
    match dot(p, r) {
        _ => println!("None"),
    }
}
