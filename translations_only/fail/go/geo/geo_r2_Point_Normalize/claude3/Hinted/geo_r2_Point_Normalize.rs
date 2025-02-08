
use std::f64;

#[derive(Debug, Clone, Copy)]
struct GeoR2Point {
    x: f64,
    y: f64,
}

impl GeoR2Point {
    fn normalize(self) -> GeoR2Point {
        if self.x == 0.0 && self.y == 0.0 {
            return self;
        }
        self.mul(1.0 / self.norm())
    }

    fn norm(self) -> f64 {
        f64::hypot(self.x, self.y)
    }

    fn mul(self, m: f64) -> GeoR2Point {
        GeoR2Point {
            x: m * self.x,
            y: m * self.y,
        }
    }
}

fn main() {
    let p1 = GeoR2Point {
        x: 2.804115934775642e-306,
        y: -1.187637765753075e+308,
    };
    let p2 = p1.normalize();
    println!("{:?}", p2);

    let p3 = GeoR2Point {
        x: -7.303147401230546e+182,
        y: -7.65260242482936e+304,
    };
    let p4 = p3.normalize();
    println!("{:?}", p4);

    let p5 = GeoR2Point {
        x: 2.1062466034e-313,
        y: 1.48560427983e-313,
    };
    let p6 = p5.normalize();
    println!("{:?}", p6);

    let p7 = GeoR2Point {
        x: 5.422324326077e-312,
        y: 2.640102242165914e-309,
    };
    let p8 = p7.normalize();
    println!("{:?}", p8);
}
