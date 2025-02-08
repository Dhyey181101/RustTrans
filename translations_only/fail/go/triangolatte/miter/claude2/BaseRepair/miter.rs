
use std::f64::consts::PI;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn subtract(a: Point, b: Point) -> Point {
    Point {
        x: a.x - b.x,
        y: a.y - b.y
    }
}

fn add(a: Point, b: Point) -> Point {
    Point {
        x: a.x + b.x, 
        y: a.y + b.y
    }
}

fn dot(a: Point, b: Point) -> f64 {
    a.x * b.x + a.y * b.y
}

fn normalize(p: Point) -> Point {
    let norm = (p.x.powi(2) + p.y.powi(2)).sqrt();
    Point {
        x: p.x / norm,
        y: p.y / norm
    }
}

fn scale(p: Point, f: f64) -> Point {
    let norm = (p.x.powi(2) + p.y.powi(2)).sqrt();
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f
    }
}

fn calculate_normals(x: f64, y: f64) -> [Point; 2] {
    [
        normalize(Point {
            x: y,
            y: -x,
        }),
        normalize(Point {
            x: -y,
            y: x,
        })
    ]
}

fn miter(points: Vec<Point>, width: f64) -> Vec<f64> {
    let mut triangles = Vec::with_capacity(points.len() * 12);

    let half_width = width / 2.0;
    
    let mut dx;
    let mut dy;

    let mut miter1 = Point{x: 0.0, y: 0.0};
    let mut miter2 = Point{x: 0.0, y: 0.0};

    let mut n1: [Point; 2];
    let mut n2: [Point; 2];

    let p0 = points[0];
    dx = subtract(points[1], p0).x;
    dy = subtract(points[1], p0).y;

    n2 = calculate_normals(dx, dy);
    miter2 = scale(n2[0], half_width);

    for i in 1..points.len()-1 {
        n1 = n2;
        miter1 = miter2;
        
        let p_curr = points[i];
        let p_next = points[i+1];
        dx = subtract(p_next, p_curr).x;
        dy = subtract(p_next, p_curr).y;
        
        n2 = calculate_normals(dx, dy);
        
        let tangent = add(normalize(subtract(p_next, p_curr)), 
            normalize(subtract(p_curr, points[i-1])));
        let unit_miter = Point {
            x: -tangent.y,
            y: tangent.x
        };
        
        let miter_len = half_width / dot(n1[0], unit_miter);
        miter2 = scale(unit_miter, miter_len);
        
        let v0 = subtract(p_curr, miter2).x;
        let v1 = subtract(p_curr, miter2).y;
        let v2 = subtract(points[i-1], miter1).x;
        let v3 = subtract(points[i-1], miter1).y;
        let v4 = add(points[i-1], miter1).x;
        let v5 = add(points[i-1], miter1).y;
        let v6 = add(points[i-1], miter1).x;
        let v7 = add(points[i-1], miter1).y;
        let v8 = add(p_curr, miter2).x;
        let v9 = add(p_curr, miter2).y;
        let v10 = subtract(p_curr, miter2).x;
        let v11 = subtract(p_curr, miter2).y;
        
        triangles.extend_from_slice(&[
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11
        ]);
    }

    let n = points.len();
    let last_miter = scale(n2[0], half_width);
    
    let p_last = points[n-1];
    let v0 = subtract(p_last, last_miter).x;
    let v1 = subtract(p_last, last_miter).y;
    let v2 = subtract(points[n-2], miter1).x;
    let v3 = subtract(points[n-2], miter1).y;
    let v4 = add(points[n-2], miter1).x;
    let v5 = add(points[n-2], miter1).y;
    let v6 = add(points[n-2], miter1).x;
    let v7 = add(points[n-2], miter1).y;
    let v8 = add(p_last, last_miter).x; 
    let v9 = add(p_last, last_miter).y;
    let v10 = subtract(p_last, last_miter).x;
    let v11 = subtract(p_last, last_miter).y;
    
    triangles.extend_from_slice(&[
        v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11
    ]);

    triangles
}

