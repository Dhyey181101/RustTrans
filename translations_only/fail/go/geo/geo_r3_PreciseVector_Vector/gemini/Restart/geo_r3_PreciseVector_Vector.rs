
use std::f64::consts::SQRT_2;

fn vector(v: geo_r3_PreciseVector) -> geo_r3_Vector {
    let x = v.X;
    let y = v.Y;
    let z = v.Z;
    normalize(geo_r3_Vector { x, y, z })
}

fn normalize(v: geo_r3_Vector) -> geo_r3_Vector {
    let n2 = norm2(&v);
    if n2 == 0.0 {
        geo_r3_Vector { x: 0.0, y: 0.0, z: 0.0 }
    } else {
        mul(&v, 1.0 / n2.sqrt())
    }
}

fn norm2(v: &geo_r3_Vector) -> f64 {
    dot(v, v)
}

fn dot(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

fn mul(v: &geo_r3_Vector, m: f64) -> geo_r3_Vector {
    geo_r3_Vector {
        x: v.x * m,
        y: v.y * m,
        z: v.z * m,
    }
}

#[derive(Debug)]
struct geo_r3_PreciseVector {
    X: f64,
    Y: f64,
    Z: f64,
}

#[derive(Debug)]
struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64,
}
