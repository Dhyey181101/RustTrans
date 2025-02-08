

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;

pub fn is_valid(c: &Cap) -> bool {
    is_unit(&c.center.vector) && c.radius <= GEO_S1_STRAIGHT_CHORD_ANGLE
}

pub fn is_unit(v: &Vector) -> bool {
    const EPSILON: f64 = 5e-14;
    (norm2(v) - 1.0).abs() <= EPSILON
}

pub fn norm2(v: &Vector) -> f64 {
    dot(v, v)
}

pub fn dot(v: &Vector, ov: &Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Debug)]
pub struct Cap {
    pub center: Point,
    pub radius: f64,
}

#[derive(Debug)]
pub struct Point {
    pub vector: Vector,
}

#[derive(Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug)]
pub struct ChordAngle(pub f64);

#[derive(Debug)]
pub struct TypeTag(pub u32);

#[derive(Debug)]
pub enum WedgeRel {
    Inside,
    Outside,
    Partial,
}

#[derive(Debug)]
pub enum CrossingType {
    Crosses,
    Interior,
    Exterior,
}

#[derive(Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug)]
pub enum CellRelation {
    Interior,
    Exterior,
    Intersecting,
}

#[derive(Debug)]
pub enum ShapeIndexIteratorPos {
    Start,
    End,
}

#[derive(Debug)]
pub enum Direction {
    Positive,
    Negative,
}

#[derive(Debug)]
pub enum CrossingTarget {
    Interior,
    Exterior,
}

#[derive(Debug)]
pub enum Crossing {
    Crosses,
    Interior,
    Exterior,
}

#[derive(Debug)]
pub enum VertexModel {
    Normal,
    Normalized,
}

#[derive(Debug)]
pub enum R3Axis {
    X,
    Y,
    Z,
}

#[derive(Debug)]
pub struct Angle(pub f64);

