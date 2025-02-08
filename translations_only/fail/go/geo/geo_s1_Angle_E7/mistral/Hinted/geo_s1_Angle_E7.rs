
use std::fmt;

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (std::f64::consts::PI / 180.0) * GEO_S1_RADIAN;

fn geo_s1_round(val: f64) -> i32 {
    if val < 0.0 {
        return (val - 0.5) as i32;
    }
    return (val + 0.5) as i32;
}

struct GeoS1Angle(f64);

impl GeoS1Angle {
    fn degrees(&self) -> f64 {
        return self.0 / GEO_S1_DEGREE;
    }

    fn e7(&self) -> i32 {
        return geo_s1_round(self.degrees() * 1e7);
    }
}

impl fmt::Display for GeoS1Angle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

fn main() {
    let angle_values = vec![-3.1299492428431496e+105, -2.804628407694207e+101, 1.7482478293972592e-256, 3.2358e-319];
    for value in angle_values {
        match GeoS1Angle(value) {
            GeoS1Angle(v) if v.is_nan() || v.is_infinite() => println!("Input is invalid, crash gracefully"),
            _ => println!("{}", GeoS1Angle(value).e7()),
        }
    }
}
