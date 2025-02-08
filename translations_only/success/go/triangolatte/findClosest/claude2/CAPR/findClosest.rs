

use std::f64::consts::PI;

struct Point {
    x: f64,
    y: f64
}

fn find_closest(m: &Point, k: &Point, p_index: usize, outer: &[Point]) -> usize {
    let mut reflex = Vec::new();
    let n = outer.len();
    
    for i in 0..n {
        let not_inside = !is_inside_triangle(m, k, &outer[p_index], &outer[i]);
        let prev = wrap(i.wrapping_sub(1).try_into().unwrap(), n);
        let next = wrap(i.wrapping_add(1).try_into().unwrap(), n);
        
        if not_inside || !is_reflex(&outer[prev], &outer[i], &outer[next]) {
            continue;
        }
        
        reflex.push(i);
    }
    
    let mut closest = 0;
    let mut max_dist = 0.0;
    
    for r in &reflex {
        let i = *r;
        let dist = point_dist_sq(&outer[i], &outer[closest]);
        
        if dist > max_dist {
            closest = i;
            max_dist = dist;
        }
    }
    
    closest
}

fn is_inside_triangle(a: &Point, b: &Point, c: &Point, p: &Point) -> bool {
    let abcp = (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y);
    let abpa = (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y);
    let bcpb = (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y);

    abcp >= 0.0 && abpa >= 0.0 && bcpb >= 0.0
}

fn wrap(i: isize, n: usize) -> usize {
    ((i % n as isize + n as isize) % n as isize) as usize
}

fn is_reflex(a: &Point, b: &Point, c: &Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0    
}

fn point_dist_sq(p1: &Point, p2: &Point) -> f64 {
    (p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)
}

