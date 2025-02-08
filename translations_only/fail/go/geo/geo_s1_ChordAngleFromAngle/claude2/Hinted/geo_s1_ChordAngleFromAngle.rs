
mod test {
    use std::f64::{self, INFINITY};

    struct Angle(f64);

    struct ChordAngle(f64);

    const NEGATIVE_CHORD_ANGLE: ChordAngle = ChordAngle(-1.0);

    fn chord_angle_from_angle(a: Angle) -> ChordAngle {
        if a.0 < 0.0 {
            return NEGATIVE_CHORD_ANGLE;
        }
        if a.0.is_infinite() {
            return inf_chord_angle();
        }
        let l = 2.0 * a.0.sin_cos().0.min(std::f64::consts::PI).sin();
        ChordAngle(l * l)
    }

    fn inf_chord_angle() -> ChordAngle {
        ChordAngle(INFINITY)
    }

    impl Angle {
        fn radians(&self) -> f64 {
            self.0
        }
    }
}
