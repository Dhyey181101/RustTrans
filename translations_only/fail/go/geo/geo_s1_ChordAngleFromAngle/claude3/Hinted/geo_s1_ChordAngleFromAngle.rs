
use std::f64::consts::PI;
use std::f64;

const GEO_S1_NEGATIVE_CHORD_ANGLE: f64 = -1.0;

fn geo_s1_chord_angle_from_angle(a: f64) -> f64 {
    if a < 0.0 {
        return GEO_S1_NEGATIVE_CHORD_ANGLE;
    }
    if a.is_infinite() {
        return geo_s1_inf_chord_angle();
    }
    let l = 2.0 * (0.5 * a.min(PI)).sin();
    l * l
}

fn geo_s1_inf_chord_angle() -> f64 {
    f64::INFINITY
}

fn main() {
    let test_cases = vec![
        5.839374350639015e+199,
        5.037644329914593e-05,
        f64::NAN,
    ];

    for case in test_cases {
        let result = geo_s1_chord_angle_from_angle(case);
        println!("{}", result);
    }
}
