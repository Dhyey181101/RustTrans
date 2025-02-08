
mod test {
    const GEO_S1_RADIAN: GeoS1Angle = 1.0;

    struct GeoS2Cap {
        radius: GeoS1ChordAngle,
    }

    struct GeoS2Point {
        _vector: GeoR3Vector,
    }

    struct GeoR3Vector {}

    type GeoS1ChordAngle = f64;
    type GeoS1Angle = f64;

    impl GeoS2Cap {
        fn radius(&self) -> GeoS1Angle {
            angle(self.radius)
        }
    }

    fn angle(c: GeoS1ChordAngle) -> GeoS1Angle {
        if c < 0.0 {
            return -1.0 * GEO_S1_RADIAN;
        }
        if is_infinity(c) {
            return inf_angle();
        }
        2.0 * (0.5 * f64::sqrt(c as f64)).asin()
    }

    fn is_infinity(c: GeoS1ChordAngle) -> bool {
        c.is_infinite()
    }

    fn inf_angle() -> GeoS1Angle {
        f64::INFINITY
    }
}
