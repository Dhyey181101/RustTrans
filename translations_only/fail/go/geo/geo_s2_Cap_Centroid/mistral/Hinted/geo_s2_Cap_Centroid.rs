
use std::fmt;
use std::f64;

#[derive(Copy, Clone, Default)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn mul(&self, m: f64) -> GeoR3Vector {
        GeoR3Vector {
            x: m * self.x,
            y: m * self.y,
            z: m * self.z,
        }
    }
}

#[derive(Copy, Clone, Default)]
struct GeoS1ChordAngle(f64);

#[derive(Copy, Clone, Default)]
struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

#[derive(Copy, Clone, Default)]
struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

impl GeoS2Cap {
    fn centroid(&self) -> GeoS2Point {
        if self.is_empty() {
            return GeoS2Point::default();
        }
        let r = 1.0 - 0.5 * self.height();
        GeoS2Point {
            geo_r3_vector: self.center.geo_r3_vector.mul(r * self.area()),
        }
    }

    fn is_empty(&self) -> bool {
        self.radius.0 < 0.0
    }

    fn height(&self) -> f64 {
        0.5 * self.radius.0
    }

    fn area(&self) -> f64 {
        2.0 * f64::consts::PI * f64::max(0.0, self.height())
    }
}

impl fmt::Debug for GeoR3Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GeoR3Vector({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Debug for GeoS1ChordAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GeoS1ChordAngle({})", self.0)
    }
}

impl fmt::Debug for GeoS2Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GeoS2Point({:?})", self.geo_r3_vector)
    }
}

impl fmt::Debug for GeoS2Cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GeoS2Cap({:?}, {:?})", self.center, self.radius)
    }
}

fn main() {
    let cap0 = GeoS2Cap {
        center: GeoS2Point {
            geo_r3_vector: GeoR3Vector {
                x: -7.268135864719839e+182,
                y: 2.99151672413e-313,
                z: 1.2823273988760575e+294,
            },
        },
        radius: GeoS1ChordAngle(-6.97118615904384e+182),
    };

    let cap1 = GeoS2Cap {
        center: GeoS2Point {
            geo_r3_vector: GeoR3Vector {
                x: 1.397376668266446e-309,
                y: 0.0,
                z: 2.313197663218592e-23,
            },
        },
        radius: GeoS1ChordAngle(0.0),
    };

    println!("{:?}", cap0.centroid());
    println!("{:?}", cap1.centroid());
}
