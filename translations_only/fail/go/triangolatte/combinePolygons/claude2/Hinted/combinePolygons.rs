
use std::f64::consts::PI;

#[derive(PartialEq, Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn combine_polygons(outer: Vec<Point>, inner: Vec<Point>) -> Result<Vec<Point>, String> {
    let m = &inner[find_max_x_index(&inner)];

    let k = find_k(m, &outer).map_err(|_| "could not find visible vertex".to_string())?;

    let mut visible_index = -1;
    for (i, point) in outer.iter().enumerate() {
        if point.x == k.x && point.y == k.y {
            visible_index = i as i32;
        }
    }

    let mut result = Vec::with_capacity(outer.len() + inner.len() + 2);
    for i in 0..visible_index as usize + 1 {
        result.push(outer[i].clone());
    }
    for i in 0..inner.len() {
        result.push(inner[(find_max_x_index(&inner) + i) % inner.len()].clone());
    }
    result.push(m.clone());
    result.push(outer[visible_index as usize].clone());
    for i in visible_index as usize + 1..outer.len() {
        result.push(outer[i].clone());
    }

    Ok(result)
}

fn find_max_x_index(v: &Vec<Point>) -> usize {
    let mut max_x = 0.0;
    let mut max_index = 0;
    for (i, p) in v.iter().enumerate() {
        if p.x > max_x {
            max_x = p.x;
            max_index = i;
        }
    }
    max_index
}

fn find_k(m: &Point, outer: &[Point]) -> Result<Point, ()> {
    let mut k = Point { x: 0.0, y: 0.0 };

    for window in outer.windows(2) {
        if window[0].y > m.y || window[1].y < m.y {
            continue;
        }

        let v1 = Point {
            x: m.x - window[0].x,
            y: m.y - window[0].y,
        };
        let v2 = Point {
            x: window[1].x - window[0].x,
            y: window[1].y - window[0].y,
        };

        let t1 = v2.x * v1.y - v2.y * v1.x;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 - m.x < k.x {
                k.x = t1 + m.x;
                k.y = m.y;
                return Ok(k);
            }
        } else {
            return Err(());
        }
    }

    Err(())
}

