
use std::f64::consts::PI;

fn calculate_normals(x: f64, y: f64) -> [Point; 2] {
    [
        Point {
            x: y,
            y: -x,
        }
        .normalize(),
        Point {
            x: -y,
            y: x,
        }
        .normalize(),
    ]
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn normalize(&self) -> Point {
        let norm = (self.x * self.x + self.y * self.y).sqrt();
        Point {
            x: self.x / norm,
            y: self.y / norm,
        }
    }

    fn scale(&self, f: f64) -> Point {
        Point {
            x: self.x * f,
            y: self.y * f,
        }
    }
}

fn main() {
    let tests = [
        (5.59536776764676e-310, -1.881009792271374e-37),
        (1.3604779505626352e+187, 7.119798244613948e-308),
        (2.2003667914e-312, -1.4378275183250288e-189),
        (7.62811347923e-312, 9.450211629936659e-260),
    ];

    for (x, y) in tests {
        let normals = calculate_normals(x, y);
        println!("{:?}", normals);
    }
}
