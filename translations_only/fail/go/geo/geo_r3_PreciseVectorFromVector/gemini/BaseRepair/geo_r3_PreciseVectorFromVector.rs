
use std::f64::consts::PI;

pub fn geo_r3_PreciseVectorFromVector(v: geo_r3_Vector) -> geo_r3_PreciseVector {
    geo_r3_NewPreciseVector(v.X, v.Y, v.Z)
}

pub fn geo_r3_NewPreciseVector(x: f64, y: f64, z: f64) -> geo_r3_PreciseVector {
    geo_r3_PreciseVector {
        X: Box::new(x),
        Y: Box::new(y),
        Z: Box::new(z),
    }
}

pub fn geo_r3_precFloat(f: f64) -> Box<f64> {
    Box::new(f)
}

#[derive(Debug, Clone)]
pub struct geo_r3_Vector {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}

#[derive(Debug, Clone)]
pub struct geo_r3_PreciseVector {
    pub X: Box<f64>,
    pub Y: Box<f64>,
    pub Z: Box<f64>,
}
