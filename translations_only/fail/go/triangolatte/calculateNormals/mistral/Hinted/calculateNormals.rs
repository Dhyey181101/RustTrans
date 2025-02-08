
use std::f64;
use std::option::Option;
use std::option::Option::Some;
use std::option::Option::None;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn calculate_normals(x: f64, y: f64) -> [Point; 2] {
    let point1 = Point { x: y, y: -x };
    let point2 = Point { x: -y, y: x };
    let mut normalized_point1 = point1.normalize();
    let mut normalized_point2 = point2.normalize();
    [normalized_point1, normalized_point2]
}

impl Point {
    fn normalize(mut self) -> Point {
        let norm = self.norm();
        Point {
            x: self.x / norm,
            y: self.y / norm,
        }
    }

    fn scale(&mut self, f: f64) {
        let norm = self.norm();
        self.x = self.x / norm * f;
        self.y = self.y / norm * f;
    }

    fn norm(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }
}

fn main() {
    let input1 = Point { x: 5.565994959707e-311, y: -1.6445217394237498e+289 };
    let normals1 = calculate_normals(input1.x, input1.y);
    println!("{:?}", normals1);

    let input2 = Point { x: 3.384897063317365e+125, y: -1.1694655295448413e-125 };
    let normals2 = calculate_normals(input2.x, input2.y);
    println!("{:?}", normals2);

    let mut input3 = Point { x: 6.397091006966572e-308, y: 0.0 };
    if input3.x.is_nan() || input3.x.is_infinite() || input3.y.is_nan() || input3.y.is_infinite() {
        panic!("Input is invalid, crash gracefully");
    }

    let mut input4 = Point { x: 6.151918997986153e-304, y: 1.88028727641e-312 };
    if input4.x.is_nan() || input4.x.is_infinite() || input4.y.is_nan() || input4.y.is_infinite() {
        panic!("Input is invalid, crash gracefully");
    }
}
