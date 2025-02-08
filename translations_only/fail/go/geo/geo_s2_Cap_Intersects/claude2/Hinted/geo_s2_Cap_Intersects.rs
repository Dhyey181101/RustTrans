

use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

fn intersects(c: Box<Cap>, other: Box<Cap>) -> bool {
    if is_empty(&*c) || is_empty(&*other) {
        return false;
    }

    c.radius + other.radius >= chord_angle_between_points(&c.center, &other.center)
}

fn is_empty(c: &Cap) -> bool {
    c.radius < 0.0
}

fn add(c: f64, other: f64) -> f64 {
    if other == 0.0 {
        return c;
    }
    
    if c + other >= GEO_S1_MAX_LENGTH2 {
        return GEO_S1_STRAIGHT_CHORD_ANGLE;
    }

    let x = c * (1.0 - 0.25 * other);
    let y = other * (1.0 - 0.25 * c);

    f64::min(GEO_S1_MAX_LENGTH2, x + y + 2.0 * (x * y).sqrt())
}

fn chord_angle_between_points(x: &Point, y: &Point) -> f64 {
    f64::min(4.0, vector_norm_squared(&sub(&x.v, &y.v)))  
}

fn sub(v: &Vector, ov: &Vector) -> Vector {
    Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn vector_norm_squared(v: &Vector) -> f64 {
    vector_dot(v, v)
}

fn vector_dot(v: &Vector, ov: &Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

struct Cap {
    center: Point,
    radius: f64,
}

struct Point {
    v: Vector,
}

struct Vector {
    x: f64,
    y: f64,
    z: f64,
}


