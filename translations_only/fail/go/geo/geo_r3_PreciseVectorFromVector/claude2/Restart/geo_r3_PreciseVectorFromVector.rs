
use std::f64;

const GEO_R3_PREC: u64 = u64::MAX;

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64  
}

struct GeoR3PreciseVector {
    x: Box<BigInt>,
    y: Box<BigInt>,
    z: Box<BigInt>
}

struct BigInt;

fn geo_r3_precise_vector_from_vector(v: GeoR3Vector) -> GeoR3PreciseVector {
    geo_r3_new_precise_vector(v.x, v.y, v.z)  
}

fn geo_r3_new_precise_vector(x: f64, y: f64, z: f64) -> GeoR3PreciseVector {
    GeoR3PreciseVector {
        x: Box::new(geo_r3_prec_float(x)),
        y: Box::new(geo_r3_prec_float(y)),
        z: Box::new(geo_r3_prec_float(z))
    }
}

fn geo_r3_prec_float(_f: f64) -> BigInt {
    BigInt
}

