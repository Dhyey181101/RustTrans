

use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = PI / 2.0;

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

impl GeoS2Cap {
    fn is_valid(&self) -> bool {
        self.center.geo_r3_vector.is_unit() && self.radius <= GEO_S1_STRAIGHT_CHORD_ANGLE
    }
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn geo_r3_vector_norm2(v: &GeoR3Vector) -> f64 {
    let GeoR3Vector { x, y, z } = v;
    x * x + y * y + z * z
}

fn geo_r3_vector_dot(v1: &GeoR3Vector, v2: &GeoR3Vector) -> f64 {
    let GeoR3Vector { x: x1, y: y1, z: z1 } = v1;
    let GeoR3Vector { x: x2, y: y2, z: z2 } = v2;
    x1 * x2 + y1 * y2 + z1 * z2
}

impl GeoR3Vector {
    fn is_unit(&self) -> bool {
        const EPSILON: f64 = 5e-14;
        let norm2 = geo_r3_vector_norm2(self);
        (norm2 - 1.0).abs() <= EPSILON
    }
}

struct GeoS2Point {
    geo_r3_vector: Box<GeoR3Vector>,
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

#[allow(non_camel_case_types)]
type geo_s1_Angle = f64;

