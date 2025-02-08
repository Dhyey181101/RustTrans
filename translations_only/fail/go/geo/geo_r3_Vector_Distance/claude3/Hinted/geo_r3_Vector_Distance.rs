
use std::f64;

#[derive(Clone, Copy)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn distance(self, other: GeoR3Vector) -> f64 {
        self.sub(other).norm()
    }

    fn sub(self, other: GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn norm(self) -> f64 {
        self.dot(self).sqrt()
    }

    fn dot(self, other: GeoR3Vector) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}

fn main() {
    let v1 = GeoR3Vector {
        x: 1.6923451878987595e+47,
        y: -2.6659711148925736e-11,
        z: 5.0629762778021e-309,
    };
    let v2 = GeoR3Vector {
        x: -5.33790637072029e+49,
        y: 2.1583758371206377e-257,
        z: 2.9932783200042e-86,
    };
    let dist = v1.distance(v2);
    println!("{}", dist); // Output: 5.354829822599277e+49

    let v3 = GeoR3Vector {
        x: -1.347468719243487e+28,
        y: -3.554188480929314e-132,
        z: 5.066398905391783e-309,
    };
    let v4 = GeoR3Vector {
        x: -1.2998370412523514e+28,
        y: 2.049365284049e-311,
        z: 0.0,
    };
    let dist = v3.distance(v4);
    println!("{}", dist); // Output: 4.7631677991135576e+26

    let v5 = GeoR3Vector {
        x: 2.1583759960297893e-257,
        y: -3.5243806439773887e-132,
        z: -1.4290355327400427e-284,
    };
    let v6 = GeoR3Vector {
        x: 3.217223502676588e-86,
        y: 1.3605952567032271e-129,
        z: 5.357281454306369e+222,
    };
    let dist = v5.distance(v6);
    println!("{}", dist); // Output: 5.357281454306369e+222

    let v7 = GeoR3Vector {
        x: 5.3414114e-317,
        y: -3.635361568980709e-132,
        z: -3.577163270497399e-132,
    };
    let v8 = GeoR3Vector {
        x: 2.225073858507203e-308,
        y: 1.0311500780800456e+82,
        z: 3.5109479738953626e+227,
    };
    let dist = v7.distance(v8);
    println!("{}", dist); // Output: 3.5109479738953626e+227
}
