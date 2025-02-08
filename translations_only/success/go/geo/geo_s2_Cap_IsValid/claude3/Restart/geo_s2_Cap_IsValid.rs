

use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = PI / 2.0;

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

fn is_valid(cap: &GeoS2Cap) -> bool {
    is_unit(&cap.center.geo_r3_vector) && cap.radius <= GEO_S1_STRAIGHT_CHORD_ANGLE
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn is_unit(vector: &GeoR3Vector) -> bool {
    const EPSILON: f64 = 5e-14;
    (norm2(vector) - 1.0).abs() <= EPSILON
}

fn norm2(vector: &GeoR3Vector) -> f64 {
    dot(vector, vector)
}

fn dot(v1: &GeoR3Vector, v2: &GeoR3Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

type GeoS1ChordAngle = f64;

#[allow(non_camel_case_types)]
type geo_s2_typeTag = u32;

#[allow(non_camel_case_types)]
type geo_s2_WedgeRel = i32;

#[allow(non_camel_case_types)]
type geo_s2_CrossingType = i32;

#[allow(non_camel_case_types)]
type geo_s2_axis = i32;

#[allow(non_camel_case_types)]
type geo_s2_CellRelation = i32;

#[allow(non_camel_case_types)]
type geo_s2_ShapeIndexIteratorPos = i32;

#[allow(non_camel_case_types)]
type geo_s2_Direction = i32;

#[allow(non_camel_case_types)]
type geo_s2_crossingTarget = i32;

#[allow(non_camel_case_types)]
type geo_s2_Crossing = i32;

#[allow(non_camel_case_types)]
type geo_s2_VertexModel = i32;

#[allow(non_camel_case_types)]
type geo_r3_Axis = i32;

type GeoS1Angle = f64;

