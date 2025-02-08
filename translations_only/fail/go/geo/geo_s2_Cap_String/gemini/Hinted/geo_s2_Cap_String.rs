
use std::f64::consts::PI;
use std::f64::INFINITY;

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (PI / 180.0) * GEO_S1_RADIAN;

fn geo_s2_cap_to_string(cap: &geo_s2_cap) -> String {
    format!(
        "[Center={}, Radius={}]",
        cap.center,
        cap.radius.angle().to_degrees()
    )
}

fn geo_s2_cap_radius(cap: &geo_s2_cap) -> f64 {
    cap.radius.angle()
}

fn geo_s1_chord_angle_angle(chord_angle: f64) -> f64 {
    if chord_angle < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if chord_angle.is_infinite() {
        return INFINITY;
    }
    2.0 * f64::asin(0.5 * f64::sqrt(chord_angle))
}

fn geo_s1_chord_angle_is_infinity(chord_angle: f64) -> bool {
    chord_angle.is_infinite()
}

fn geo_s1_inf_angle() -> f64 {
    INFINITY
}

fn geo_s1_angle_degrees(angle: f64) -> f64 {
    angle / GEO_S1_DEGREE
}

#[derive(Debug)]
struct geo_s2_cap {
    center: String,
    radius: geo_s1_chord_angle,
}

#[derive(Debug)]
struct geo_s1_chord_angle(f64);

#[derive(Debug)]
struct geo_s1_angle(f64);

impl geo_s1_chord_angle {
    fn angle(&self) -> f64 {
        geo_s1_chord_angle_angle(self.0)
    }

    fn is_infinity(&self) -> bool {
        geo_s1_chord_angle_is_infinity(self.0)
    }
}

impl geo_s1_angle {
    fn degrees(&self) -> f64 {
        geo_s1_angle_degrees(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_s2_cap_to_string() {
        let cap = geo_s2_cap {
            center: String::from(""),
            radius: geo_s1_chord_angle(8.933e-321),
        };
        assert_eq!(geo_s2_cap_to_string(&cap), "[Center=, Radius=9.451299845423241e-161]");
    }

    #[test]
    fn test_geo_s2_cap_radius() {
        let cap = geo_s2_cap {
            center: String::from(""),
            radius: geo_s1_chord_angle(5.539128795439826e-307),
        };
        assert_eq!(geo_s2_cap_radius(&cap), 7.442532361662814e-154);
    }

    #[test]
    fn test_geo_s1_chord_angle_angle() {
        assert_eq!(geo_s1_chord_angle_angle(-3.37817857989693e-135), 0.0);
        assert_eq!(geo_s1_chord_angle_angle(3.486677463719918e+30), 0.0);
        assert_eq!(
            geo_s1_chord_angle_angle(8.933e-321),
            9.451299845423241e-161
        );
    }

    #[test]
    fn test_geo_s1_chord_angle_is_infinity() {
        assert!(geo_s1_chord_angle_is_infinity(INFINITY));
        assert!(!geo_s1_chord_angle_is_infinity(0.0));
    }

    #[test]
    fn test_geo_s1_inf_angle() {
        assert_eq!(geo_s1_inf_angle(), INFINITY);
    }

    #[test]
    fn test_geo_s1_angle_degrees() {
        assert_eq!(geo_s1_angle_degrees(1.0), 57.29577951308232);
    }
}
