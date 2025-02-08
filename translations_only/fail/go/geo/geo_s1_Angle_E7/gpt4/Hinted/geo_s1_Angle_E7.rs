
const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = std::f64::consts::PI / 180.0 * GEO_S1_RADIAN;

fn e7(angle: f64) -> i32 {
    geo_s1_round(degrees(angle) * 1e7)
}

fn degrees(angle: f64) -> f64 {
    angle / GEO_S1_DEGREE
}

fn geo_s1_round(val: f64) -> i32 {
    if val.is_infinite() {
        panic!("Input is invalid, crash gracefully");
    } else if val < 0.0 {
        (val - 0.5) as i32
    } else {
        (val + 0.5) as i32
    }
}

fn main() {
    // Example usage
    println!("{}", e7(6.010989184800804e+250)); // This will panic
}
