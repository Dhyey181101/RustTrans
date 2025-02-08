
use std::f64::consts::PI;

struct GeoS2Point {
    x: f64,
    y: f64,
    z: f64,   
}

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: f64,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn add_point(mut cap: GeoS2Cap, p: GeoS2Point) -> GeoS2Cap {
    if cap.radius < 0.0 {
        cap.center = Box::new(p);
        cap.radius = 0.0;
        return cap;
    }
    
    let new_rad = chord_angle_between_points(&*cap.center, &p);
    if new_rad > cap.radius {
        cap.radius = new_rad;
    }
    cap
}

fn is_empty(cap: &GeoS2Cap) -> bool {
    cap.radius < 0.0   
}

fn chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> f64 {
    let v = point_to_vector(x);
    let ov = point_to_vector(y);
    vector_norm_2(&vector_sub(&v, &ov))
}

fn vector_sub(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn vector_norm_2(v: &GeoR3Vector) -> f64 {
    vector_dot(v, v)
}

fn vector_dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z  
}

fn point_to_vector(p: &GeoS2Point) -> GeoR3Vector {
    GeoR3Vector {
        x: p.x,
        y: p.y,
        z: p.z,
    }
}

