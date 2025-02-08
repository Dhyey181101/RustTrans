
const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;

pub fn is_valid(c: &geo_s2_Cap) -> bool {
    is_unit(&c.center.geo_r3_Vector) && c.radius <= GEO_S1_STRAIGHT_CHORD_ANGLE
}

pub fn is_unit(v: &geo_r3_Vector) -> bool {
    const EPSILON: f64 = 5e-14;
    (norm2(v) - 1.0).abs() <= EPSILON
}

pub fn norm2(v: &geo_r3_Vector) -> f64 {
    dot(v, v)
}

pub fn dot(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
    v.X * ov.X + v.Y * ov.Y + v.Z * ov.Z
}

#[derive(Clone, Copy)]
pub struct geo_s2_Cap {
    pub center: geo_s2_Point,
    pub radius: f64,
}

#[derive(Clone, Copy)]
pub struct geo_s2_Point {
    pub geo_r3_Vector: geo_r3_Vector,
}

#[derive(Clone, Copy)]
pub struct geo_r3_Vector {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}

#[derive(Clone, Copy)]
pub struct geo_s1_ChordAngle(pub f64);

#[derive(Clone, Copy)]
pub struct geo_s2_typeTag(pub u32);

#[derive(Clone, Copy)]
pub struct geo_s2_WedgeRel(pub i32);

#[derive(Clone, Copy)]
pub struct geo_s2_CrossingType(pub i32);

#[derive(Clone, Copy)]
pub struct geo_s2_axis(pub i32);

#[derive(Clone, Copy)]
pub struct geo_s2_CellRelation(pub i32);

#[derive(Clone, Copy)]
pub struct geo_s2_ShapeIndexIteratorPos(pub i32);

#[derive(Clone, Copy)]
pub struct geo_s2_Direction(pub i32);

#[derive(Clone, Copy)]
pub struct geo_s2_crossingTarget(pub i32);

#[derive(Clone, Copy)]
pub struct geo_s2_Crossing(pub i32);

#[derive(Clone, Copy)]
pub struct geo_s2_VertexModel(pub i32);

#[derive(Clone, Copy)]
pub struct geo_r3_Axis(pub i32);

#[derive(Clone, Copy)]
pub struct geo_s1_Angle(pub f64);

