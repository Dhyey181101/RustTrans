
fn cube(x: f64) -> f64 {
    x * x * x
}

fn main() {
    let test_cases = vec![
        (-2.315844028150888e+77 as f64),
        (-0.031162112358515515 as f64),
        (-4.538015467766058e+279 as f64),
        (5.3567386217750754e+302 as f64),
    ];

    for test_case in test_cases {
        match cube(test_case) {
            result if result.is_nan() || result.is_infinite() => println!("None"),
            result => println!("{}", result),
        }
    }
}
