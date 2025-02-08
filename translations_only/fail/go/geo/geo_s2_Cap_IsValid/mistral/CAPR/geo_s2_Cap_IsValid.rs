

use std::f64;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;

const EPSILON: f64 = 5e-14;

#[derive(Debug, Clone, Copy)]
struct Geo_r3_Vector {
x: f64,
y: f64,
z: f64,
}

impl Geo_r3_Vector {
fn norm2(&self) -> f64 {
self.x * self.x + self.y * self.y + self.z * self.z
}
}

fn is_unit_vector(v: &Geo_r3_Vector) -> bool {
let v_norm2 = v.norm2();
(v_norm2 - 1.0).abs() <= EPSILON
}

fn dot(v: &Geo_r3_Vector, ov: &Geo_r3_Vector) -> f64 {
v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Debug, Clone, Copy)]
struct Geo_s2_Point {
geo_r3_vector: Geo_r3_Vector,
}

#[derive(Debug, Clone, Copy)]
struct Geo_s1_ChordAngle(f64);

#[derive(Debug, Clone, Copy)]
struct Geo_s2_typeTag(u32);

#[derive(Debug, Clone, Copy)]
struct Geo_s2_WedgeRel(i32);

#[derive(Debug, Clone, Copy)]
struct Geo_s2_CrossingType(i32);

#[derive(Debug, Clone, Copy)]
struct Geo_s2_axis(i32);

#[derive(Debug, Clone, Copy)]
struct Geo_s2_CellRelation(i32);

#[derive(Debug, Clone, Copy)]
struct Geo_s2_ShapeIndexIteratorPos(i32);

#[derive(Debug, Clone, Copy)]
struct Geo_s2_Direction(i32);

#[derive(Debug, Clone, Copy)]
struct Geo_s2_crossingTarget(i32);

#[derive(Debug, Clone, Copy)]
struct Geo_s2_Crossing(i32);

#[derive(Debug, Clone, Copy)]
struct Geo_s2_VertexModel(i32);

#[derive(Debug, Clone, Copy)]
struct Geo_r3_Axis(i32);

#[derive(Debug, Clone, Copy)]
struct Geo_s1_Angle(f64);

