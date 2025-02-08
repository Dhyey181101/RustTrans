

use std::f64;
use std::boxed::Box;

struct GeoR2Point {
    x: f64,
    y: f64,
}

fn norm(p: &GeoR2Point) -> f64 {
    f64::hypot(p.x, p.y)
}

fn normalize(mut p: GeoR2Point) -> GeoR2Point {
    if p.x == 0.0 && p.y == 0.0 {
        return p;
    }
    let norm = norm(&p);
    GeoR2Point {
        x: p.x / norm,
        y: p.y / norm,
    }
}

fn mul(m: f64, p: &GeoR2Point) -> GeoR2Point {
    GeoR2Point {
        x: m * p.x,
        y: m * p.y,
    }
}

fn main() {
    // Test cases go here
    let p1 = GeoR2Point { x: -7.951351022940651e-63, y: 3.2640991631741123e+25 };
    let n1 = normalize(p1);
    assert!((n1.x - -2.4360016731870694e-88).abs() < 1e-20);
    assert!((n1.y - 1.0).abs() < 1e-20);

    let p2 = GeoR2Point { x: -1.1981735312177333e-59, y: -5.683097295773726e-63 };
    let n2 = normalize(p2);
    assert!((n2.x - -0.9999998875134309).abs() < 1e-20);
    assert!((n2.y - -0.0004743133201019504).abs() < 1e-20);

    let p3 = GeoR2Point { x: 8.3657562063273e-310, y: 1.428061786848e-312 };
    let n3 = normalize(p3);

    let p4 = GeoR2Point { x: 1.026770103374564e-309, y: 3.2379e-319 };
    let n4 = normalize(p4);
}

