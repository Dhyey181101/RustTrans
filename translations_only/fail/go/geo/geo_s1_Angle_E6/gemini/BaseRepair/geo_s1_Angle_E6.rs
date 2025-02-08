
const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (std::f64::consts::PI / 180.0) * GEO_S1_RADIAN;

fn geo_s1_round(val: f64) -> i32 {
    if val < 0.0 {
        return (val - 0.5) as i32;
    }
    return (val + 0.5) as i32;
}

fn geo_s1_angle_e6(a: f64) -> i32 {
    return geo_s1_round(a * 1e6);
}

fn geo_s1_angle_degrees(a: f64) -> f64 {
    return a / GEO_S1_DEGREE;
}
