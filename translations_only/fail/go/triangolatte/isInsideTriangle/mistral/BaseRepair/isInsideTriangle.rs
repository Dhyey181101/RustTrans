

use std::cmp::Ordering;

pub struct Point {
    x: f64,
    y: f64,
}

pub fn is_inside_triangle(a: &Point, b: &Point, c: &Point, p: &Point) -> bool {
    let ab_cross_cp = (b.x - a.x) * (c.y - p.y) - (c.x - a.x) * (b.y - p.y);
    let bc_cross_ap = (c.x - b.x) * (a.y - p.y) - (a.x - b.x) * (c.y - p.y);
    let ca_cross_bp = (a.x - c.x) * (b.y - p.y) - (b.x - c.x) * (a.y - p.y);

    if ab_cross_cp.partial_cmp(&0.0).unwrap() == Ordering::Less || ab_cross_cp.partial_cmp(&0.0).is_none() {
        return false;
    }

    if bc_cross_ap.partial_cmp(&0.0).unwrap() == Ordering::Less || bc_cross_ap.partial_cmp(&0.0).is_none() {
        return false;
    }

    if ca_cross_bp.partial_cmp(&0.0).unwrap() == Ordering::Less || ca_cross_bp.partial_cmp(&0.0).is_none() {
        return false;
    }

    is_on_segment(a, b, p) && is_on_segment(b, c, p) && is_on_segment(c, a, p)
}

fn is_on_segment(a: &Point, b: &Point, p: &Point) -> bool {
    let ab_length_sq = (b.x - a.x).powf(2.0) + (b.y - a.y).powf(2.0);
    let ap_length_sq = (p.x - a.x).powf(2.0) + (p.y - a.y).powf(2.0);
    let bp_length_sq = (p.x - b.x).powf(2.0) + (p.y - b.y).powf(2.0);

    (ab_length_sq - ap_length_sq - bp_length_sq).abs() < 1e-10
}

