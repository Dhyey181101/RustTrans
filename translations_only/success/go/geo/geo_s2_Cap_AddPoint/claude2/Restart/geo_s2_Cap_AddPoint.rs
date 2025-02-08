
use std::f64::consts::PI;

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    vector: Box<GeoR3Vector>, 
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

type GeoS1ChordAngle = f64;

fn add_point(mut cap: GeoS2Cap, p: GeoS2Point) -> GeoS2Cap {
    if cap.radius < 0.0 {
        cap.center = p;
        cap.radius = 0.0;
        return cap;
    }
    
    let new_rad = chord_angle_between_points(&cap.center, &p);
    if new_rad > cap.radius {
        cap.radius = new_rad;
    }
    cap
}

fn is_empty(cap: &GeoS2Cap) -> bool {
    cap.radius < 0.0
}

fn chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> GeoS1ChordAngle {
    let diff = sub_vectors(&x.vector, &y.vector);
    GeoS1ChordAngle::min(4.0, norm_squared(&diff))  
}

fn sub_vectors(x: &Box<GeoR3Vector>, y: &Box<GeoR3Vector>) -> Box<GeoR3Vector> {
    Box::new(GeoR3Vector {
        x: x.x - y.x,
        y: x.y - y.y,
        z: x.z - y.z,
    })
}

fn norm_squared(v: &Box<GeoR3Vector>) -> f64 {
    dot(v, v)
}

fn dot(x: &Box<GeoR3Vector>, y: &Box<GeoR3Vector>) -> f64 {
    x.x * y.x + x.y * y.y + x.z * y.z  
}

