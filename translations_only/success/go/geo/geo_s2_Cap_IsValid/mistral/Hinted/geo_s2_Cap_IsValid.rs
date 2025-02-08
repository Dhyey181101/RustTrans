
use std::f64;

const EPSILON: f64 = 5e-14;
const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;

fn is_valid_cap(c: &Cap) -> bool {
    is_unit(&c.center.vector) && c.radius <= GEO_S1_STRAIGHT_CHORD_ANGLE
}

fn is_unit(v: &Vector) -> bool {
    (v.norm2() - 1.0).abs() <= EPSILON
}

fn norm2(v: &Vector) -> f64 {
    v.dot(v)
}

fn dot(v: &Vector, ov: &Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Debug, Copy, Clone)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn dot(&self, ov: &Vector) -> f64 {
        dot(self, ov)
    }

    fn norm2(&self) -> f64 {
        norm2(self)
    }

    fn is_unit(&self) -> bool {
        is_unit(self)
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    vector: Vector,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            vector: Vector { x, y, z },
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Cap {
    center: Point,
    radius: f64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum S2TypeTag {
    WedgeRel = 0,
    CrossingType = 1,
    Axis = 2,
    CellRelation = 3,
    ShapeIndexIteratorPos = 4,
    Direction = 5,
    CrossingTarget = 6,
    Crossing = 7,
    VertexModel = 8,
    R3Axis = 9,
    S1Angle = 10,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum S2WedgeRel {
    Left = 0,
    Right = 1,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum S2CrossingType {
    Edge = 0,
    Vertex = 1,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum S2Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum S2CellRelation {
    Inside = 0,
    Boundary = 1,
    Outside = 2,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum S2ShapeIndexIteratorPos {
    Current = 0,
    Next = 1,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum S2Direction {
    Positive = 0,
    Negative = 1,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum S2CrossingTarget {
    Target = 0,
    Source = 1,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum S2Crossing {
    Cross = 0,
    Same = 1,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum S2VertexModel {
    Face = 0,
    Edge = 1,
    Vertex = 2,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum R3Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct S1Angle(f64);
