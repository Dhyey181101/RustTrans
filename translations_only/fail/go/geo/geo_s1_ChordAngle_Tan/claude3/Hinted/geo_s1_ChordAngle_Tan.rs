
use std::f64::consts::PI;

fn tan(c: f64) -> f64 {
    sin(c) / cos(c)
}

fn sin(c: f64) -> f64 {
    (sin2(c)).sqrt()
}

fn sin2(c: f64) -> f64 {
    // Let a be the (non-squared) chord length, and let A be the corresponding
    // half-angle (a = 2*sin(A)). The formula below can be derived from:
    //   sin(2*A) = 2 * sin(A) * cos(A)
    //   cos^2(A) = 1 - sin^2(A)
    // This is much faster than converting to an angle and computing its sine.
    c * (1.0 - 0.25 * c)
}

fn cos(c: f64) -> f64 {
    // cos(2*A) = cos^2(A) - sin^2(A) = 1 - 2*sin^2(A)
    1.0 - 0.5 * c
}

fn main() {
    let test_cases = vec![
        9.557922771591467e-299,
        3.785766995733679e-270,
    ];

    for test_case in test_cases {
        if test_case < 0.0 || test_case > 2.0 * PI {
            println!("Input is invalid, crash gracefully");
        } else {
            let tan_result = tan(test_case);
            let sin_result = sin(test_case);
            let sin2_result = sin2(test_case);
            let cos_result = cos(test_case);

            println!("tan: {}", tan_result);
            println!("sin: {}", sin_result);
            println!("sin2: {}", sin2_result);
            println!("cos: {}", cos_result);
        }
    }
}
