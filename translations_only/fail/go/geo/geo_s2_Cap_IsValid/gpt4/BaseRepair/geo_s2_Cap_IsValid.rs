
const GEO_S1_STRAIGHT_CHORD_ANGLE: GeoS1ChordAngle = GeoS1ChordAngle(4.0);

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

impl GeoS2Cap {
    fn is_valid(&self) -> bool {
        self.center.0.is_unit() && self.radius.0 <= GEO_S1_STRAIGHT_CHORD_ANGLE.0
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

    fn dot(&self, other: &GeoR3Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

struct GeoS2Point(GeoR3Vector);

struct GeoS1ChordAngle(f64);

enum GeoS2TypeTag {
    Value(u32),
}

enum GeoS2WedgeRel {
    Value(i32),
}

enum GeoS2CrossingType {
    Value(i32),
}

enum GeoS2Axis {
    Value(i32),
}

enum GeoS2CellRelation {
    Value(i32),
}

enum GeoS2ShapeIndexIteratorPos {
    Value(i32),
}

enum GeoS2Direction {
    Value(i32),
}

enum GeoS2CrossingTarget {
    Value(i32),
}

enum GeoS2Crossing {
    Value(i32),
}

enum GeoS2VertexModel {
    Value(i32),
}

enum GeoR3Axis {
    Value(i32),
}

struct GeoS1Angle(f64);
