

use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = PI / 90.0;

struct GeoS2Cap {
    center: Box<GeoS2Point>,
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

impl GeoR3Vector {
    fn is_unit(&self) -> bool {
        const EPSILON: f64 = 5e-14;
        (self.norm2() - 1.0).abs() <= EPSILON
    }

    fn norm2(&self) -> f64 {
        self.dot(self)
    }

    fn dot(&self, ov: &GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

type GeoS1ChordAngle = f64;

#[derive(Clone, Copy)]
struct GeoS2TypeTag(u32);

#[derive(Clone, Copy)]
struct GeoS2WedgeRel(i32);

#[derive(Clone, Copy)]
struct GeoS2CrossingType(i32);

#[derive(Clone, Copy)]
struct GeoS2Axis(i32);

#[derive(Clone, Copy)]
struct GeoS2CellRelation(i32);

#[derive(Clone, Copy)]
struct GeoS2ShapeIndexIteratorPos(i32);

#[derive(Clone, Copy)]
struct GeoS2Direction(i32);

#[derive(Clone, Copy)]
struct GeoS2CrossingTarget(i32);

#[derive(Clone, Copy)]
struct GeoS2Crossing(i32);

#[derive(Clone, Copy)]
struct GeoS2VertexModel(i32);

#[derive(Clone, Copy)]
struct GeoR3Axis(i32);

type GeoS1Angle = f64;

