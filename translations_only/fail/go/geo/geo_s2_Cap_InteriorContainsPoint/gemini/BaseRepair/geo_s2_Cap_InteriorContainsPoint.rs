
const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;

pub fn interior_contains_point(cap: &Cap, p: &Point) -> bool {
    is_full(cap) || chord_angle_between_points(&cap.center, p) < cap.radius
}

pub fn is_full(cap: &Cap) -> bool {
    cap.radius == GEO_S1_STRAIGHT_CHORD_ANGLE
}

pub fn chord_angle_between_points(x: &Point, y: &Point) -> f64 {
    f64::min(4.0, sub(&x.geo_r3_vector, &y.geo_r3_vector).norm2())
}

fn sub(x: &Vector, y: &Vector) -> Vector {
    Vector {
        x: x.x - y.x,
        y: x.y - y.y,
        z: x.z - y.z,
    }
}

fn norm2(v: &Vector) -> f64 {
    v.x * v.x + v.y * v.y + v.z * v.z
}

fn dot(x: &Vector, y: &Vector) -> f64 {
    x.x * y.x + x.y * y.y + x.z * y.z
}

#[derive(Copy, Clone)]
pub struct Cap {
    center: Point,
    radius: f64,
}

#[derive(Copy, Clone)]
pub struct Point {
    geo_r3_vector: Vector,
}

#[derive(Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn norm2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}
