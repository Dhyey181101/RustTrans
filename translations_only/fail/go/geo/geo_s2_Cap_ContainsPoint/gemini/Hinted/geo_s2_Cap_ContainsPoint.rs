

use std::f64::consts::PI;

fn contains_point(cap: Cap, p: Point) -> bool {
    chord_angle_between_points(&cap.center, &p) <= cap.radius
}

fn chord_angle_between_points(x: &Point, y: &Point) -> f64 {
    f64::min(4.0, (x.sub(y)).norm2()).sqrt() * PI / 2.0
}

impl Point {
    fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn norm2(&self) -> f64 {
        self.dot(self)
    }

    fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[derive(Copy, Clone)]
struct Cap {
    center: Point,
    radius: f64,
}

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[test]
fn test_contains_point() {
    let cap = Cap {
        center: Point {
            x: 1.390671161325008e-309,
            y: 1.612226962694291e+265,
            z: 3.2379e-319,
        },
        radius: 1.0410277202464601e-42,
    };
    let p = Point {
        x: 9.52682703076063e+139,
        y: 0.0,
        z: 0.0,
    };
    assert_eq!(contains_point(cap, p), false);

    let cap = Cap {
        center: Point {
            x: 1.390671161325008e-309,
            y: 9.98011009403164e-310,
            z: 5.056318096e-315,
        },
        radius: 1.988684538624e-312,
    };
    let p = Point {
        x: 0.0,
        y: 0.0,
        z: -5.486124068793696e+303,
    };
    assert_eq!(contains_point(cap, p), false);

    let cap = Cap {
        center: Point {
            x: 1.390671161325008e-309,
            y: -8.968312911262657e-44,
            z: 2.701779002112805e-42,
        },
        radius: 8.344028523031688e-308,
    };
    let p = Point {
        x: 0.0,
        y: 1.489438e-317,
        z: 0.0,
    };
    assert_eq!(contains_point(cap, p), true);

    let cap = Cap {
        center: Point {
            x: 2.891970570575669e+143,
            y: 0.0,
            z: 1.1125369292536007e-308,
        },
        radius: 5.884403334537659e-62,
    };
    let p = Point {
        x: 7.72329135e-315,
        y: 7.469425265145e-312,
        z: 5.06153621916954e-310,
    };
    assert_eq!(contains_point(cap, p), true);
}

