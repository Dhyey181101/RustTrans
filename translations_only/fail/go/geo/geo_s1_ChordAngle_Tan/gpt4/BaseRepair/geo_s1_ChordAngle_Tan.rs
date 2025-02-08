
struct GeoS1ChordAngle(f64);

impl GeoS1ChordAngle {
    fn tan(&self) -> f64 {
        self.sin() / self.cos()
    }

    fn sin(&self) -> f64 {
        (self.sin2()).sqrt()
    }

    fn sin2(&self) -> f64 {
        let c = self.0;
        c * (1.0 - 0.25 * c)
    }

    fn cos(&self) -> f64 {
        let c = self.0;
        1.0 - 0.5 * c
    }
}

fn main() {}
