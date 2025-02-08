
use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;

fn geo_s2_chord_angle_between_points(x: &geo_s2_point, y: &geo_s2_point) -> f64 {
    let v = x.sub(y);
    f64::min(GEO_S1_STRAIGHT_CHORD_ANGLE, v.norm2().sqrt())
}

fn geo_s2_cap_interior_contains_point(c: &geo_s2_cap, p: &geo_s2_point) -> bool {
    c.radius == GEO_S1_STRAIGHT_CHORD_ANGLE || geo_s2_chord_angle_between_points(&c.center, p) < c.radius
}

#[derive(Debug, Clone, Copy)]
struct geo_s2_point {
    x: f64,
    y: f64,
    z: f64,
}

impl geo_s2_point {
    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn norm2(&self) -> f64 {
        self.dot(self)
    }

    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[derive(Debug, Clone, Copy)]
struct geo_s2_cap {
    center: geo_s2_point,
    radius: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_s2_chord_angle_between_points() {
        let x = geo_s2_point {
            x: 4.198672567229607e-140,
            y: 1.4574439370836964e-105,
            z: 3.650626088884612e-103,
        };
        let y = geo_s2_point {
            x: 1.4275161023838926e-105,
            y: 1.4260258154373855e-105,
            z: 1.4260258159703532e-105,
        };
        assert_eq!(geo_s2_chord_angle_between_points(&x, &y), 1.4260391220948502e-105);
    }

    #[test]
    fn test_geo_s2_cap_interior_contains_point() {
        let c = geo_s2_cap {
            center: geo_s2_point {
                x: 2.34629074607964e-310,
                y: 1.3750698347387358e+23,
                z: 1.0821468733728352e-23,
            },
            radius: 1.4260258159703532e-105,
        };
        let p = geo_s2_point {
            x: 1.4339030416743683e-105,
            y: 1.4260665659766346e-105,
            z: 1.4268774079383549e-105,
        };
        assert_eq!(geo_s2_cap_interior_contains_point(&c, &p), false);
    }
}

