
const GEO_S2_NUM_FACES: u64 = 6;
const GEO_S2_MAX_LEVEL: u64 = 30;
const GEO_S2_POS_BITS: u64 = 2 * GEO_S2_MAX_LEVEL + 1;
const GEO_S2_WRAP_OFFSET: u64 = GEO_S2_NUM_FACES << GEO_S2_POS_BITS;

struct GeoS2CellID(u64);

impl GeoS2CellID {
    fn prev_wrap(&self) -> GeoS2CellID {
        let p = self.prev();
        if p.0 < GEO_S2_WRAP_OFFSET {
            return p;
        }
        GeoS2CellID(p.0 + GEO_S2_WRAP_OFFSET)
    }

    fn prev(&self) -> GeoS2CellID {
        GeoS2CellID(self.0 - self.lsb() << 1)
    }

    fn lsb(&self) -> u64 {
        self.0 & (!self.0).wrapping_add(1)
    }
}
