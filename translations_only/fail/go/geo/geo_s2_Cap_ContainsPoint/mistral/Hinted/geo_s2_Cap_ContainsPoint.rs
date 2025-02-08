

use std::fmt;

#[derive(Copy, Clone, Default)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn sub(&self, ov: GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x - ov.x,
            y: self.y - ov.y,
            z: self.z - ov.z,
        }
    }

    fn norm2(&self) -> f64 {
        self.dot(*self)
    }

    fn dot(&self, ov: GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}

#[derive(Copy, Clone, Default)]
struct GeoS2Point(GeoR3Vector);

#[derive(Copy, Clone, Default)]
struct GeoS1ChordAngle(f64);

#[derive(Copy, Clone, Default)]
struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

impl GeoS2Cap {
    fn contains_point(&self, p: GeoS2Point) -> bool {
        let dist_sq = f64::min(4.0, self.center.0.sub(p.0).norm2());
        dist_sq.sqrt() <= self.radius.0
    }
}

fn geo_s2_chord_angle_between_points(x: GeoR3Vector, y: GeoR3Vector) -> GeoS1ChordAngle {
    let dist_sq = f64::min(4.0, x.sub(y).norm2());
    GeoS1ChordAngle(dist_sq.sqrt())
}

impl fmt::Debug for GeoR3Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Debug for GeoS2Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GeoS2Point({:?})", self.0)
    }
}

impl fmt::Debug for GeoS1ChordAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GeoS1ChordAngle({})", self.0)
    }
}

impl fmt::Debug for GeoS2Cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GeoS2Cap({:?}, {:?})", self.center, self.radius)
    }
}

fn main() {
    let cap = GeoS2Cap {
        center: GeoS2Point(GeoR3Vector {
            x: 8.09520590311e-312,
            y: 592.408203125,
            z: -8.08634922409991e-174,
        }),
        radius: GeoS1ChordAngle(1.9808e-319),
    };

    let p = GeoS2Point(GeoR3Vector {
        x: 5.0511374e-317,
        y: 0.0,
        z: 0.0,
    });

    println!("{:?}.contains_point({:?}) = {:?}", cap, p, cap.contains_point(p));

    let cap = GeoS2Cap {
        center: GeoS2Point(GeoR3Vector {
            x: 1.0564754980432676e-307,
            y: -4.512392510013289e-87,
            z: 3.0135156695747363e+296,
        }),
        radius: GeoS1ChordAngle(1.6585863093e-314),
    };

    let p = GeoS2Point(GeoR3Vector {
        x: 1.392644617879987e-309,
        y: -2.6632045401665453e+96,
        z: -7.722268125228326e-84,
    });

    println!("{:?}.contains_point({:?}) = {:?}", cap, p, cap.contains_point(p));
}

