
const GEO_S2_NUM_FACES: u64 = 6;
const GEO_S2_MAX_LEVEL: u64 = 30;
const GEO_S2_POS_BITS: u64 = 2 * GEO_S2_MAX_LEVEL + 1;
const GEO_S2_WRAP_OFFSET: u64 = GEO_S2_NUM_FACES << GEO_S2_POS_BITS;

struct GeoS2CellID(u64);

impl GeoS2CellID {
    fn lsb(&self) -> u64 {
        self.0 & (!self.0).wrapping_add(1)
    }
}

fn prev_wrap(ci: Box<GeoS2CellID>) -> Box<GeoS2CellID> {
    let p = prev(ci);
    if p.0 < GEO_S2_WRAP_OFFSET {
        return p;
    }
    Box::new(GeoS2CellID(p.0 + GEO_S2_WRAP_OFFSET))
}

fn prev(ci: Box<GeoS2CellID>) -> Box<GeoS2CellID> {
    Box::new(GeoS2CellID(ci.0 - ci.lsb() << 1))
}
