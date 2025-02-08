
use std::f64;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    let check_side = |a: Point, b: Point, p: Point| {
        (b.x - p.x) * (a.y - p.y) - (a.x - p.x) * (b.y - p.y)
    };
    check_side(c, a, p) >= 0.0 && check_side(a, b, p) >= 0.0 && check_side(b, c, p) >= 0.0
}

fn main() {
    let p1 = Point { x: 7.4561107008e-312, y: 5.626512268278315e+149 };
    let p2 = Point { x: 2.5673651826636406e+151, y: 2.5673651826636406e+151 };
    let p3 = Point { x: 2.566086514457431e+151, y: 2.5673651826636406e+151 };
    let p = Point { x: 9.040815532771263e-131, y: 1.33685771882e-312 };
    println!("{}", is_inside_triangle(p1, p2, p3, p));

    let p1 = Point { x: 1.2248204475510398e-255, y: 8.344026969402005e-309 };
    let p2 = Point { x: 3.026687417978888e+267, y: 2.0885320418321696e-306 };
    let p3 = Point { x: 8.814425663403416e-280, y: 3.8600057750114834e-304 };
    let p = Point { x: -1.5208732530527268e-209, y: 6.0177e-320 };
    println!("{}", is_inside_triangle(p1, p2, p3, p));

    let p1 = Point { x: 7.4561107008e-312, y: 2.5673560058323477e+151 };
    let p2 = Point { x: 2.5673651826636406e+151, y: 2.5673651826636406e+151 };
    let p3 = Point { x: 4.611846888335064e-302, y: -1.4918728543202125e+233 };
    let p = Point { x: 2.567297752905788e+151, y: -1.0773165018460756e+236 };
    println!("{}", is_inside_triangle(p1, p2, p3, p));

    let p1 = Point { x: 7.4561107008e-312, y: 2.5673562741076137e+151 };
    let p2 = Point { x: 2.5673651826636406e+151, y: 2.5673664313630808e+151 };
    let p3 = Point { x: 4.28749985504828e+152, y: 1.0835551704008931e+116 };
    let p = Point { x: 2.3148855333193033e+21, y: -3.105036184601418e+231 };
    println!("{}", is_inside_triangle(p1, p2, p3, p));
}
