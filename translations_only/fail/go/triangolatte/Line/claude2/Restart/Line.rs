
use std::f64::consts::PI;

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,  
}

enum Joint {
    Normal,
    Miter
}

fn line(joint: Joint, points: Vec<Point>, width: f64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    match joint {
        Joint::Normal => Ok(normal(points, width)),
        Joint::Miter => Ok(miter(points, width)),
        _ => Err("Unrecognized joint type".into())
    }
}

fn normal(points: Vec<Point>, width: f64) -> Vec<f64> {
    let mut triangles = Vec::with_capacity(points.len() * 12);

    for i in 0..points.len()-1 {
        let dx = points[i+1].x - points[i].x;
        let dy = points[i+1].y - points[i].y;
        
        let n1 = Point { x: dy, y: -dx };
        let n2 = Point { x: -dy, y: dx };
        
        let n1 = scale(n1, width / 2.0);
        let n2 = scale(n2, width / 2.0);
        
        let v0 = points[i+1].x + n2.x;
        let v1 = points[i+1].y + n2.y;
        
        let v2 = points[i].x + n2.x;
        let v3 = points[i].y + n2.y;
        
        let v4 = points[i].x + n1.x;
        let v5 = points[i].y + n1.y;
        
        let v6 = points[i].x + n1.x;
        let v7 = points[i].y + n1.y;
        
        let v8 = points[i+1].x + n1.x;
        let v9 = points[i+1].y + n1.y;
        
        let v10 = points[i+1].x + n2.x;  
        let v11 = points[i+1].y + n2.y;
        
        triangles.extend_from_slice(&[
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11
        ]);
    }

    triangles
}

fn scale(p: Point, f: f64) -> Point {
    let norm = (p.x*p.x + p.y*p.y).sqrt();
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f
    }
}  

fn add(p: Point, r: Point) -> Point {
    Point {
        x: p.x + r.x,
        y: p.y + r.y
    }
}

fn miter(points: Vec<Point>, width: f64) -> Vec<f64> {
    let mut triangles = Vec::with_capacity(points.len() * 12);
    
    let width = width / 2.0;
    
    let mut dx = points[1].x - points[0].x;
    let mut dy = points[1].y - points[0].y;
    
    let n2 = calculate_normals(dx, dy);
    let mut miter2 = scale(n2[0], width);

    for i in 1..points.len()-1 {
        dx = points[i+1].x - points[i].x;
        dy = points[i+1].y - points[i].y;

        let n2 = calculate_normals(dx, dy);
        
        let tangent = add(
            scale(sub(points[i+1], points[i]), 1.0 / norm(sub(points[i+1], points[i]))),
            scale(sub(points[i], points[i-1]), 1.0 / norm(sub(points[i], points[i-1])))
        );
        
        let unit_miter = Point { x: -tangent.y, y: tangent.x };
        
        let miter_length = width / dot(unit_miter, n2[0]);
        miter2 = scale(unit_miter, miter_length);
        
        let v0 = points[i].x - miter2.x;
        let v1 = points[i].y - miter2.y;
        
        let v2 = points[i-1].x - miter2.x; 
        let v3 = points[i-1].y - miter2.y;
        
        let v4 = points[i-1].x + miter2.x;
        let v5 = points[i-1].y + miter2.y;
        
        let v6 = points[i-1].x + miter2.x;
        let v7 = points[i-1].y + miter2.y;
        
        let v8 = points[i].x + miter2.x;
        let v9 = points[i].y + miter2.y;
        
        let v10 = points[i].x - miter2.x;  
        let v11 = points[i].y - miter2.y;
        
        triangles.extend_from_slice(&[
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11 
        ]);
    }

    triangles
}

fn calculate_normals(x: f64, y: f64) -> [Point; 2] {
    [
        scale(Point { x: y, y: -x }, 1.0),
        scale(Point { x: -y, y: x }, 1.0)
    ]
}

fn norm(p: Point) -> f64 {
    (p.x*p.x + p.y*p.y).sqrt()
}

fn sub(p: Point, r: Point) -> Point {
    Point {
        x: p.x - r.x,
        y: p.y - r.y
    }
}

fn dot(p: Point, r: Point) -> f64 {
    p.x * r.x + p.y * r.y  
}
