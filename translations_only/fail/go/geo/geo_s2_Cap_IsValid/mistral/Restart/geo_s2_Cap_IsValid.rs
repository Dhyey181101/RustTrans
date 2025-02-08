

use std::f64;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;
const EPSILON: f64 = 5e-14;

fn is_unit_vector(v: &Geo_r3_Vector) -> bool {
let v_norm2 = (v.x * v.x + v.y * v.y + v.z * v.z).abs();
(v_norm2 - 1.0).abs() <= EPSILON
}

fn norm2(v: &Geo_r3_Vector) -> f64 {
v.x * v.x + v.y * v.y + v.z * v.z
}

fn dot(v: &Geo_r3_Vector, ov: &Geo_r3_Vector) -> f64 {
v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Debug, Clone, Copy)]
struct Geo_s2_Cap {
center: Geo_s2_Point,
radius: Geo_s1_ChordAngle,
}

#[derive(Debug, Clone, Copy)]
struct Geo_s2_Point {
geo_r3_vector: Geo_r3_Vector,
}

#[derive(Debug, Clone, Copy)]
struct Geo_r3_Vector {
x: f64,
y: f64,
z: f64,
}

#[derive(Debug, Clone, Copy)]
struct Geo_s1_ChordAngle(f64);

#[derive(Debug, Clone, Copy)]
enum Geo_s2_typeTag {}

#[derive(Debug, Clone, Copy)]
enum Geo_s2_WedgeRel {}

#[derive(Debug, Clone, Copy)]
enum Geo_s2_CrossingType {}

#[derive(Debug, Clone, Copy)]
enum Geo_s2_axis {}

#[derive(Debug, Clone, Copy)]
enum Geo_s2_CellRelation {}

#[derive(Debug, Clone, Copy)]
enum Geo_s2_ShapeIndexIteratorPos {}

#[derive(Debug, Clone, Copy)]
enum Geo_s2_Direction {}

#[derive(Debug, Clone, Copy)]
enum Geo_s2_crossingTarget {}

#[derive(Debug, Clone, Copy)]
enum Geo_s2_Crossing {}

#[derive(Debug, Clone, Copy)]
enum Geo_s2_VertexModel {}

#[derive(Debug, Clone, Copy)]
enum Geo_r3_Axis {}

#[derive(Debug, Clone, Copy)]
struct Geo_s1_Angle(f64);

