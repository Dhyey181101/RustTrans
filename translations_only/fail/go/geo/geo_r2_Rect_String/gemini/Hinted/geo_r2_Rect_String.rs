
pub fn stringify_geo_r2_rect(r: &geo_r2_rect) -> String {
    format!("[Lo{}, Hi{}]", r.lo().x, r.hi().x)
}

pub fn lo_geo_r2_rect(r: &geo_r2_rect) -> geo_r2_point {
    geo_r2_point {
        x: r.x.lo,
        y: r.y.lo,
    }
}

pub fn hi_geo_r2_rect(r: &geo_r2_rect) -> geo_r2_point {
    geo_r2_point {
        x: r.x.hi,
        y: r.y.hi,
    }
}

#[derive(Debug)]
pub struct geo_r2_rect {
    pub x: geo_r1_interval,
    pub y: geo_r1_interval,
}

#[derive(Debug)]
pub struct geo_r1_interval {
    pub lo: f64,
    pub hi: f64,
}

#[derive(Debug)]
pub struct geo_r2_point {
    pub x: f64,
    pub y: f64,
}

impl geo_r2_rect {
    pub fn lo(&self) -> geo_r2_point {
        geo_r2_point {
            x: self.x.lo,
            y: self.y.lo,
        }
    }

    pub fn hi(&self) -> geo_r2_point {
        geo_r2_point {
            x: self.x.hi,
            y: self.y.hi,
        }
    }
}
