
mod test {
    use std::f64;

    #[derive(Clone, Copy)]
    struct GeoR3Vector {
        x: f64,
        y: f64,
        z: f64,
    }

    #[derive(Clone, Copy)]
    struct GeoS2Point {
        geo_r3_vector: GeoR3Vector,
    }

    #[derive(Clone, Copy)]
    struct GeoS1ChordAngle(f64);

    #[derive(Clone, Copy)]
    struct GeoS2Cap {
        center: GeoS2Point,
        radius: GeoS1ChordAngle,
    }

    impl GeoR3Vector {
        fn sub(self, ov: GeoR3Vector) -> GeoR3Vector {
            GeoR3Vector {
                x: self.x - ov.x,
                y: self.y - ov.y,
                z: self.z - ov.z,
            }
        }

        fn norm2(self) -> f64 {
            self.dot(self)
        }

        fn dot(self, ov: GeoR3Vector) -> f64 {
            self.x * ov.x + self.y * ov.y + self.z * ov.z
        }
    }

    fn geo_s2_chord_angle_between_points(x: GeoS2Point, y: GeoS2Point) -> GeoS1ChordAngle {
        GeoS1ChordAngle(f64::min(4.0, x.geo_r3_vector.sub(y.geo_r3_vector).norm2()))
    }

    impl GeoS2Cap {
        fn is_empty(self) -> bool {
            self.radius.0 < 0.0
        }

        fn add_point(mut self, p: GeoS2Point) -> GeoS2Cap {
            if self.is_empty() {
                self.center = p;
                self.radius = GeoS1ChordAngle(0.0);
                return self;
            }

            let new_rad = geo_s2_chord_angle_between_points(self.center, p);
            if new_rad.0 > self.radius.0 {
                self.radius = new_rad;
            }
            self
        }
    }
}
