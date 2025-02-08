
use std::f64::consts::PI;

#[derive(Copy, Clone)]
struct GeoS2Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone)]
struct GeoS1ChordAngle(f64);

#[derive(Copy, Clone)]
struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

fn centroid(cap: GeoS2Cap) -> GeoS2Point {
    if is_empty(cap) {
        return GeoS2Point { x: 0.0, y: 0.0, z: 0.0 };
    }
    
    let r = 1.0 - 0.5 * height(cap);
    let area = area(cap);
    mul_point(cap.center, r * area)
}

fn is_empty(cap: GeoS2Cap) -> bool {
    cap.radius .0 < 0.0
} 

fn height(cap: GeoS2Cap) -> f64 {
    0.5 * cap.radius.0
}

fn area(cap: GeoS2Cap) -> f64 {
    2.0 * PI * f64::max(0.0, height(cap))
}

fn mul_point(point: GeoS2Point, m: f64) -> GeoS2Point {
    GeoS2Point {
        x: m * point.x,
        y: m * point.y, 
        z: m * point.z
    }
}
