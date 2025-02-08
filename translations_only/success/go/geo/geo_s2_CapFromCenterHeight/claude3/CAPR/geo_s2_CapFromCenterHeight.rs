
const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

fn geo_s2_cap_from_center_height(center: Box<geo_s2_Point>, height: f64) -> Box<geo_s2_Cap> {
    Box::new(geo_s2_cap_from_center_chord_angle(
        center,
        geo_s1_chord_angle_from_squared_length(2.0 * height),
    ))
}

fn geo_s1_chord_angle_from_squared_length(length2: f64) -> f64 {
    if length2 > GEO_S1_MAX_LENGTH2 {
        GEO_S1_STRAIGHT_CHORD_ANGLE
    } else {
        length2
    }
}

fn geo_s2_cap_from_center_chord_angle(
    center: Box<geo_s2_Point>,
    radius: f64,
) -> geo_s2_Cap {
    geo_s2_Cap {
        center,
        radius,
    }
}

struct geo_s2_Cap {
    center: Box<geo_s2_Point>,
    radius: f64,
}

struct geo_s2_Point {
    geo_r3_vector: Box<geo_r3_Vector>,
}

struct geo_r3_Vector;
