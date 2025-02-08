
const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

pub fn interior_intersects(c: &Cap, other: &Cap) -> bool {
    if c.radius <= 0.0 || c.radius + other.radius <= chord_angle_between_points(&c.center, &other.center) {
        return false;
    }
    true
}

pub fn add(c: f64, other: f64) -> f64 {
    if other == 0.0 {
        return c;
    }

    let sum = c + other;
    if sum >= GEO_S1_MAX_LENGTH2 {
        return GEO_S1_STRAIGHT_CHORD_ANGLE;
    }

    let x = sum * (1.0 - 0.25 * other);
    let y = other * (1.0 - 0.25 * sum);
    f64::min(GEO_S1_MAX_LENGTH2, x + y + 2.0 * f64::sqrt(x * y))
}

pub fn chord_angle_between_points(x: &Point, y: &Point) -> f64 {
    f64::min(4.0, sub(&x.vector, &y.vector).norm2())
}

pub fn sub(v: &Vector, ov: &Vector) -> Vector {
    Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

pub fn norm2(v: &Vector) -> f64 {
    dot(v, v)
}

pub fn dot(v: &Vector, ov: &Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

pub struct Cap {
    center: Point,
    radius: f64,
}

pub struct Point {
    vector: Vector,
}

pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn norm2(&self) -> f64 {
        dot(self, self)
    }
}
