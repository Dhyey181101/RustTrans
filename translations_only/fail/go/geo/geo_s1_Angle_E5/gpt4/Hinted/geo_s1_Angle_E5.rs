
const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = std::f64::consts::PI / 180.0 * GEO_S1_RADIAN;

fn e5(a: f64) -> i32 {
    geo_s1_round(degrees(a) * 1e5)
}

fn degrees(a: f64) -> f64 {
    a / GEO_S1_DEGREE
}

fn geo_s1_round(val: f64) -> i32 {
    if val.is_infinite() || val.is_nan() {
        panic!("Input is invalid, crash gracefully");
    } else if val < 0.0 {
        (val - 0.5) as i32
    } else {
        (val + 0.5) as i32
    }
}

fn main() {
    // Example usage
    println!("{}", e5(1.0)); // Replace 1.0 with the input value
}
