
use std::f64::consts::PI;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64
}

fn find_closest(m: Point, k: Point, p_index: usize, outer: &[Point]) -> usize {
    let mut reflex = Vec::new();
    let n = outer.len();
    
    for i in 0..n {
        let not_inside = !is_inside_triangle(m, k, outer[p_index], outer[i]);
        let prev = wrap(i.wrapping_sub(1), n);
        let next = wrap(try_usize((i as u32+1) as usize).unwrap(), n);
        let not_reflex = !is_reflex(outer[prev], outer[i], outer[next]);
        
        if not_inside || not_reflex {
            continue;
        }
        
        reflex.push(i);
    }
    
    let mut closest = 0;
    let mut max_dist = 0.0;
    
    for r in &reflex {
        let i = *r;
        let dist = distance2(&outer[i], &outer[closest]);
        
        if dist > max_dist {
            closest = i;
            max_dist = dist; 
        }
    }
    
    closest
}

fn try_usize(i: usize) -> Result<usize, ()> {
    Ok(i)
}

fn wrap(i: usize, n: usize) -> usize {
    ((i % n + n) % n)  
}

fn is_reflex(a: Point, b: Point, c: Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0   
}

fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0
        && (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0
        && (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0
}

fn distance2(p1: &Point, p2: &Point) -> f64 {
    (p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)
}
