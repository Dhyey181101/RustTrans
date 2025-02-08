
use std::ops::Mul;

const GEO_R3_PREC: u32 = 100;

struct GeoR3PreciseVector {
    x: Box<BigInt>,
    y: Box<BigInt>,
    z: Box<BigInt>,
}

struct BigInt(i64);

fn geo_r3_prec_float(f: f64) -> Box<BigInt> {
    Box::new(BigInt(f as i64))  
}

fn mul_by_float64(v: GeoR3PreciseVector, f: f64) -> GeoR3PreciseVector {
    let f = geo_r3_prec_float(f);
    mul(v, f)
}

fn mul(v: GeoR3PreciseVector, f: Box<BigInt>) -> GeoR3PreciseVector {
    GeoR3PreciseVector {
        x: prec_mul(&v.x, &f),
        y: prec_mul(&v.y, &f),
        z: prec_mul(&v.z, &f),
    }
}

fn prec_mul(a: &Box<BigInt>, b: &Box<BigInt>) -> Box<BigInt> {
    Box::new(BigInt(a.0 * b.0)) 
}

