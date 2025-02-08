
const GEO_R3_PREC: u32 = 1024;
const GEO_R3_PRECISE1: f64 = 1.0;

fn geo_r3_prec_int(i: i64) -> f64 {
    i as f64
}

fn geo_r3_prec_mul(a: f64, b: f64) -> Box<f64> {
    Box::new(a * b)
}

fn geo_r3_prec_add(a: f64, b: f64) -> Box<f64> {
    Box::new(a + b)
}

struct GeoR3PreciseVector {
    x: Box<f64>,
    y: Box<f64>,
    z: Box<f64>,
}

fn geo_r3_precise_vector_is_unit(v: &GeoR3PreciseVector) -> bool {
    geo_r3_precise_vector_norm2(v) == GEO_R3_PRECISE1
}

fn geo_r3_precise_vector_norm2(v: &GeoR3PreciseVector) -> f64 {
    geo_r3_precise_vector_dot(v, v)
}

fn geo_r3_precise_vector_dot(v1: &GeoR3PreciseVector, v2: &GeoR3PreciseVector) -> f64 {
    *geo_r3_prec_add(
        *geo_r3_prec_mul(*v1.x, *v2.x),
        *geo_r3_prec_add(
            *geo_r3_prec_mul(*v1.y, *v2.y),
            *geo_r3_prec_mul(*v1.z, *v2.z),
        ),
    )
}
