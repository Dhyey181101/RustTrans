
const GEO_S1_STRAIGHT_CHORD_ANGLE: geo_s1_ChordAngle = geo_s1_ChordAngle(4.0);
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

pub fn geo_s2_cap_from_center_height(center: geo_s2_Point, height: f64) -> geo_s2_Cap {
    geo_s2_cap_from_center_chord_angle(center, geo_s1_chord_angle_from_squared_length(2.0 * height))
}

pub fn geo_s1_chord_angle_from_squared_length(length2: f64) -> geo_s1_ChordAngle {
    if length2 > GEO_S1_MAX_LENGTH2 {
        GEO_S1_STRAIGHT_CHORD_ANGLE
    } else {
        geo_s1_ChordAngle(length2)
    }
}

pub fn geo_s2_cap_from_center_chord_angle(center: geo_s2_Point, radius: geo_s1_ChordAngle) -> geo_s2_Cap {
    geo_s2_Cap {
        center,
        radius,
    }
}

#[derive(Copy, Clone)]
pub struct geo_s1_Angle(pub f64);

#[derive(Copy, Clone)]
pub struct geo_s1_ChordAngle(pub f64);

#[derive(Copy, Clone)]
pub struct geo_s2_Cap {
    pub center: geo_s2_Point,
    pub radius: geo_s1_ChordAngle,
}

#[derive(Copy, Clone)]
pub struct geo_s2_Point {
    pub geo_r3_vector: geo_r3_Vector,
}

#[derive(Copy, Clone)]
pub struct geo_r3_Vector {}
