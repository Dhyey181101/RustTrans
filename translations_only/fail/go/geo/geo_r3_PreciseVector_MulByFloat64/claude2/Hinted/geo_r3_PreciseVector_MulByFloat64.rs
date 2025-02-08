
use std::cmp::Ordering;

struct BigInt {
    value: i64,
}

impl BigInt {
    fn new(value: i64) -> BigInt {
        BigInt { value }
    }
    
    fn multiply(&self, other: &BigInt) -> BigInt {
        BigInt::new(self.value * other.value)
    }
}

enum Sign {
    Plus,
    Minus,
}

struct GeoR3PreciseVector {
    x: Box<BigInt>,
    y: Box<BigInt>,
    z: Box<BigInt>,  
}

fn geo_r3_prec_float(f: f64) -> BigInt {
    let mut big_f = BigInt::new(f as i64);
    big_f  
}

fn geo_r3_precise_vector_mul_by_float64(v: GeoR3PreciseVector, f: f64) -> GeoR3PreciseVector {
    let big_f = geo_r3_prec_float(f);
    multiply_geo_r3_precise_vector(&v, &big_f)
}

fn multiply_geo_r3_precise_vector(v: &GeoR3PreciseVector, f: &BigInt) -> GeoR3PreciseVector {
    GeoR3PreciseVector {
        x: Box::new(v.x.multiply(&f)),
        y: Box::new(v.y.multiply(&f)),
        z: Box::new(v.z.multiply(&f)),
    }
}

