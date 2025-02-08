
use std::ops::Mul;

const GEO_R3_PREC: u64 = 1024;

fn geo_r3_precise_vector_mul_by_float64(v: Box<geo_r3_PreciseVector>, f: f64) -> Box<geo_r3_PreciseVector> {
    let f_prec = geo_r3_prec_float(f);
    Box::new(geo_r3_precise_vector_mul(v, f_prec))
}

fn geo_r3_prec_float(f: f64) -> Box<Vec<u64>> {
    let mut x = Box::new(vec![0; GEO_R3_PREC as usize]);
    let mut i = 0;
    let mut f_int = f as u64;
    while f_int != 0 {
        x[i] = f_int % 10;
        f_int /= 10;
        i += 1;
    }
    x
}

fn geo_r3_precise_vector_mul(v: Box<geo_r3_PreciseVector>, f: Box<Vec<u64>>) -> geo_r3_PreciseVector {
    geo_r3_PreciseVector {
        x: geo_r3_prec_mul(v.x.clone(), &f),
        y: geo_r3_prec_mul(v.y.clone(), &f),
        z: geo_r3_prec_mul(v.z.clone(), &f),
    }
}

fn geo_r3_prec_mul(a: Box<Vec<u64>>, b: &Box<Vec<u64>>) -> Box<Vec<u64>> {
    let mut x = Box::new(vec![0; GEO_R3_PREC as usize]);
    for i in 0..GEO_R3_PREC as usize {
        for j in 0..GEO_R3_PREC as usize {
            x[(i + j) % (GEO_R3_PREC as usize)] += a[i] * b[j];
        }
    }
    x
}

#[derive(Clone)]
struct geo_r3_PreciseVector {
    x: Box<Vec<u64>>,
    y: Box<Vec<u64>>,
    z: Box<Vec<u64>>,
}
