
use std::f64::consts::PI;

#[derive(Debug)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct GeoR3PreciseVector {
    x: f64,
    y: f64,
    z: f64,
}

fn geo_r3_precise_vector_from_vector(v: GeoR3Vector) -> GeoR3PreciseVector {
    GeoR3PreciseVector {
        x: v.x,
        y: v.y,
        z: v.z,
    }
}

fn geo_r3_new_precise_vector(x: f64, y: f64, z: f64) -> GeoR3PreciseVector {
    GeoR3PreciseVector { x, y, z }
}

fn geo_r3_prec_float(f: f64) -> f64 {
    f
}

fn main() {
    let v = GeoR3Vector {
        x: 1.00795190635e-313,
        y: 9.36221502393658e-97,
        z: 6.679571501111919e-308,
    };
    let pv = geo_r3_precise_vector_from_vector(v);
    println!("{:?}", pv);
}
