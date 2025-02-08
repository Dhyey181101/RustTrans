
use std::f64::consts::PI;

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    vec: GeoR3Vector,
}

#[derive(Clone, Copy)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

type GeoS1ChordAngle = f64;

fn centroid(cap: &GeoS2Cap) -> GeoS2Point {
    if is_empty(cap) {
        return GeoS2Point {
            vec: GeoR3Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };
    }

    let center_vec = (*cap.center).vec;
    let mut result_vec = center_vec;
    let r = 1.0 - 0.5 * height(cap);
    let area = area(cap);
    mul_scalar_mut(&mut result_vec, r * area);

    GeoS2Point {
        vec: result_vec,
    }
}

fn is_empty(cap: &GeoS2Cap) -> bool {
    cap.radius < 0.0  
}

fn height(cap: &GeoS2Cap) -> f64 {
    0.5 * cap.radius
}

fn area(cap: &GeoS2Cap) -> f64 {
    2.0 * PI * cap.radius.max(0.0)
}

fn mul_scalar(vec: GeoR3Vector, scalar: f64) -> GeoR3Vector {
    GeoR3Vector {
        x: vec.x * scalar,
        y: vec.y * scalar,
        z: vec.z * scalar,
    }
}

fn mul_scalar_mut(vec: &mut GeoR3Vector, scalar: f64) {
    vec.x *= scalar;
    vec.y *= scalar;
    vec.z *= scalar;
}

