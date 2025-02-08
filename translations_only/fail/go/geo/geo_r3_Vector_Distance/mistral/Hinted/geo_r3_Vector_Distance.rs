

use std::f64;
use std::option::Option;
use std::boxed::Box;

#[derive(Clone)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn sub(v1: &GeoR3Vector, v2: &GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
    }
}

fn dot(v1: &GeoR3Vector, v2: &GeoR3Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

fn norm(v: &GeoR3Vector) -> f64 {
    dot(v, v).sqrt()
}

fn distance(v1: &GeoR3Vector, v2: &GeoR3Vector) -> f64 {
    let vsub = sub(v1, v2);
    norm(&vsub)
}

fn main() {
    let v1 = GeoR3Vector {
        x: 1.0322987202783092e-255,
        y: 1.0323033786875544e-255,
        z: 1.5490944720781013e-120,
    };
    let v2 = GeoR3Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let v3 = GeoR3Vector {
        x: 3.7632429002127815e+77,
        y: -5.486124068793689e+303,
        z: 5.975531859577e-311,
    };
    let v4 = GeoR3Vector {
        x: 4.1624948319809485e-258,
        y: 1.3963750862531205e-308,
        z: -8.914956828517144e+303,
    };

    println!("{}", distance(&v1, &v2));
    println!("{}", norm(&v3));
    println!("{}", norm(&v4));
}

