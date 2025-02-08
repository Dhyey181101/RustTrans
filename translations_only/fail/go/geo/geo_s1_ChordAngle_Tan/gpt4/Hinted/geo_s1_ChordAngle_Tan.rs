
use std::f64::consts::PI;

struct GeoS1ChordAngle(f64);

impl GeoS1ChordAngle {
    fn tan(&self) -> Option<f64> {
        let sin = self.sin()?;
        let cos = self.cos()?;
        Some(sin / cos)
    }

    fn sin(&self) -> Option<f64> {
        self.sin2().map(f64::sqrt)
    }

    fn sin2(&self) -> Option<f64> {
        if self.0 < 0.0 || self.0 > 4.0 * PI * PI {
            None
        } else {
            Some(self.0 * (1.0 - 0.25 * self.0))
        }
    }

    fn cos(&self) -> Option<f64> {
        if self.0 < 0.0 || self.0 > 4.0 * PI * PI {
            None
        } else {
            Some(1.0 - 0.5 * self.0)
        }
    }
}

fn main() {
    let examples = vec![
        5.907072453352994e-270,
        1.44e-321,
        -1.8975166787886585e+49,
        -1.6886569198074998e-307,
    ];

    for example in examples {
        let chord_angle = GeoS1ChordAngle(example);
        match chord_angle.tan() {
            Some(result) => println!("{}", result),
            None => println!("None"),
        }
    }
}
