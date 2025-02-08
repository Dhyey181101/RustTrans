
struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

type GeoS1ChordAngle = f64;

impl GeoS2Cap {
    fn add_point(&mut self, p: GeoS2Point) {
        if self.is_empty() {
            self.center = p;
            self.radius = 0.0;
        } else {
            let new_rad = geo_s2_chord_angle_between_points(&self.center, &p);
            if new_rad > self.radius {
                self.radius = new_rad;
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.radius < 0.0
    }
}

fn geo_s2_chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> GeoS1ChordAngle {
    geo_s1_chord_angle(x.geo_r3_vector.sub(&y.geo_r3_vector).norm2())
}

fn geo_s1_chord_angle(dist: f64) -> GeoS1ChordAngle {
    f64::min(4.0, dist)
}

impl GeoR3Vector {
    fn sub(&self, ov: &GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x - ov.x,
            y: self.y - ov.y,
            z: self.z - ov.z,
        }
    }

    fn norm2(&self) -> f64 {
        self.dot(self)
    }

    fn dot(&self, ov: &GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}
