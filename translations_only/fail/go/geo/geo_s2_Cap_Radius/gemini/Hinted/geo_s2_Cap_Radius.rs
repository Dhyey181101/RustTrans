
use std::f64::consts::PI;
use std::f64::INFINITY;

#[derive(Debug)]
struct GeoS2Cap {
    radius: GeoS1ChordAngle,
}

#[derive(Debug)]
struct GeoS2Point {
    geo_r3_Vector: (),
}

#[derive(Debug)]
struct GeoR3Vector {}

#[derive(Debug)]
struct GeoS1ChordAngle(f64);

#[derive(Debug)]
struct GeoS1Angle(f64);

impl GeoS2Cap {
    fn radius(&self) -> GeoS1Angle {
        self.radius.angle()
    }
}

impl GeoS1ChordAngle {
    fn angle(&self) -> GeoS1Angle {
        if self.0 < 0.0 {
            return GeoS1Angle(-1.0 * PI);
        }
        if self.0.is_infinite() {
            return GeoS1Angle(INFINITY);
        }
        GeoS1Angle(2.0 * f64::asin(0.5 * f64::sqrt(self.0)))
    }
}

fn geo_s1_inf_angle() -> GeoS1Angle {
    GeoS1Angle(INFINITY)
}

const GEO_S1_RADIAN: GeoS1Angle = GeoS1Angle(PI);

fn main() {
    let c = GeoS2Cap {
        radius: GeoS1ChordAngle(5.80233225e-315),
    };
    println!("{:?}", c.radius());

    let c = GeoS2Cap {
        radius: GeoS1ChordAngle(2.4182479527723152e-257),
    };
    println!("{:?}", c.radius());

    let c = GeoS2Cap {
        radius: GeoS1ChordAngle(-4.872246321129615e-120),
    };
    println!("{:?}", c.radius());

    let c = GeoS2Cap {
        radius: GeoS1ChordAngle(-1.1671323904059118e-117),
    };
    println!("{:?}", c.radius());
}
