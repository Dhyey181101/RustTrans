
fn add_rect(r1: geo_r2_Rect, r2: geo_r2_Rect) -> geo_r2_Rect {
    geo_r2_Rect {
        X: union(r1.X, r2.X),
        Y: union(r1.Y, r2.Y),
    }
}

fn union(i1: geo_r1_Interval, i2: geo_r1_Interval) -> geo_r1_Interval {
    if i1.is_empty() {
        i2
    } else if i2.is_empty() {
        i1
    } else {
        geo_r1_Interval {
            Lo: i1.Lo.min(i2.Lo),
            Hi: i1.Hi.max(i2.Hi),
        }
    }
}

#[derive(Debug)]
struct geo_r2_Rect {
    X: geo_r1_Interval,
    Y: geo_r1_Interval,
}

#[derive(Debug)]
struct geo_r1_Interval {
    Lo: f64,
    Hi: f64,
}

impl geo_r1_Interval {
    fn is_empty(&self) -> bool {
        self.Lo > self.Hi
    }
}
