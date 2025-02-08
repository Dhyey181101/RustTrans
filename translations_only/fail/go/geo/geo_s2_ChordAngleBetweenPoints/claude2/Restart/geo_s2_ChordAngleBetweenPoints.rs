
#[derive(Clone)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

struct GeoS2Point {
    vector: Box<GeoR3Vector>,
}

struct GeoS1ChordAngle(f64);

fn geo_s2_chord_angle_between_points(x: GeoS2Point, y: GeoS2Point) -> GeoS1ChordAngle {
    GeoS1ChordAngle(f64::min(4.0, vector_norm2(vector_sub(x.vector, y.vector))))
}

fn vector_sub(v: Box<GeoR3Vector>, ov: Box<GeoR3Vector>) -> Box<GeoR3Vector> {
    Box::new(GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    })
}

fn vector_norm2(v: Box<GeoR3Vector>) -> f64 {
    vector_dot(v.clone(), v)  
}

fn vector_dot(v: Box<GeoR3Vector>, ov: Box<GeoR3Vector>) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

