
use std::option::Option::None;
use std::option::Option::Some;

struct Point {
    x: f64,
    y: f64,
}

fn cross(p: Point, r: Point) -> f64 {
    p.x * r.y - p.y * r.x
}

fn main() {
    let p = Point { x: 7.29112201955652e-304, y: -4.663205458474635e+304 };
    let r = Point { x: -3.051757812500001e-05, y: 8.563200242698084e-305 };
    println!("{}", cross(p, r));

    let p = Point { x: 1.633853868583e-312, y: 0.0 };
    let r = Point { x: 2.37e-322, y: 1.554196e-317 };
    if cross(p, r).abs() < 1e-300 {
        println!("0.0");
    }

    let p = Point { x: 5.314682691643932e+303, y: -8.517643009569588e+133 };
    let r = Point { x: 1.61611557065e-312, y: -1.203457044617798e+68 };
    if (p.x * r.y).abs() > 1e300 || (p.y * r.x).abs() > 1e300 {
        println!("None");
    }

    let p = Point { x: -8.77226143230349e+298, y: -1.0381602670530522e+251 };
    let r = Point { x: 1.059023548572e-311, y: -2.2338637843137761e+248 };
    if (p.x * r.y).abs() > 1e300 || (p.y * r.x).abs() > 1e300 {
        println!("None");
    }
}
