
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
struct GeoS2Point {
    geo_r3_Vector: GeoR3Vector,
}

#[derive(Debug, Clone, Copy)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn sub(&self, other: &GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn dot(&self, other: &GeoR3Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn norm2(&self) -> f64 {
        self.dot(self)
    }
}

#[derive(Debug, Clone, Copy)]
struct GeoS2Cap {
    center: GeoS2Point,
    radius: f64,
}

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

impl GeoS2Cap {
    fn interior_intersects(&self, other: GeoS2Cap) -> bool {
        if self.radius <= 0.0 || other.is_empty() {
            return false;
        }

        self.radius + other.radius > geo_s2_chord_angle_between_points(self.center, other.center)
    }

    fn is_empty(&self) -> bool {
        self.radius < 0.0
    }
}

fn geo_s2_chord_angle_between_points(x: GeoS2Point, y: GeoS2Point) -> f64 {
    x.geo_r3_Vector.sub(&y.geo_r3_Vector).norm2().min(4.0)
}

fn main() {
    let cap1 = GeoS2Cap {
        center: GeoS2Point {
            geo_r3_Vector: GeoR3Vector {
                x: -5.4861240698655e+303,
                y: 1.543e-319,
                z: -9.758548461885451e+283,
            },
        },
        radius: 0.0,
    };
    let cap2 = GeoS2Cap {
        center: GeoS2Point {
            geo_r3_Vector: GeoR3Vector {
                x: 8.0947715e-317,
                y: 1.1e-322,
                z: 0.0,
            },
        },
        radius: -1.7262643021568627e+183,
    };
    println!("{}", cap1.interior_intersects(cap2));

    let cap1 = GeoS2Cap {
        center: GeoS2Point {
            geo_r3_Vector: GeoR3Vector {
                x: 2.0090786384777806e+301,
                y: 2.684199852287e-312,
                z: 8.96831017167883e-44,
            },
        },
        radius: 2.72808271225687e-310,
    };
    let cap2 = GeoS2Cap {
        center: GeoS2Point {
            geo_r3_Vector: GeoR3Vector {
                x: 2.0421693444629802e+301,
                y: 1.5999e-319,
                z: 7.12749295116811e-67,
            },
        },
        radius: -4.538015471083013e+279,
    };
    println!("{}", cap1.interior_intersects(cap2));

    let cap1 = GeoS2Cap {
        center: GeoS2Point {
            geo_r3_Vector: GeoR3Vector {
                x: 2.481240352660348e+180,
                y: 5.993536052997881e-133,
                z: 1.1449498818352773e+108,
            },
        },
        radius: 3.886253060096822e+285,
    };
    let cap2 = GeoS2Cap {
        center: GeoS2Point {
            geo_r3_Vector: GeoR3Vector {
                x: 4.4386257263210026e-38,
                y: 9.961817178929968e-43,
                z: 5.208436914007509e-90,
            },
        },
        radius: 8.018443486237698e-91,
    };
    println!("{}", cap1.interior_intersects(cap2));

    let cap1 = GeoS2Cap {
        center: GeoS2Point {
            geo_r3_Vector: GeoR3Vector {
                x: -5.4861240698655e+303,
                y: 2.567615230854e-312,
                z: 3.63964718066364e-310,
            },
        },
        radius: 3.3960056391e-313,
    };
    let cap2 = GeoS2Cap {
        center: GeoS2Point {
            geo_r3_Vector: GeoR3Vector {
                x: 0.0,
                y: 7.2911185428784936e-304,
                z: 1.5304e-319,
            },
        },
        radius: 106496.0,
    };
    println!("{}", cap1.interior_intersects(cap2));
}
