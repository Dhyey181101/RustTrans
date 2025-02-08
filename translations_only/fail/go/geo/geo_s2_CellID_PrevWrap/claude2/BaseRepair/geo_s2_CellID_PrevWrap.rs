
use std::convert::TryFrom;

const GEO_S2_NUMFACES: u32 = 6;
const GEO_S2_MAXLEVEL: u32 = 30;
const GEO_S2_POSBITS: u32 = 2 * GEO_S2_MAXLEVEL + 1;
const GEO_S2_WRAPOFFSET: u64 = (GEO_S2_NUMFACES as u64) << GEO_S2_POSBITS;

struct CellId(u64);

impl CellId {
    fn prev_wrap(self) -> CellId {
        let p = self.prev();
        if p.0 < GEO_S2_WRAPOFFSET {
            p
        } else {
            CellId(p.0 + GEO_S2_WRAPOFFSET)
        }
    }
    
    fn prev(self) -> CellId {
        CellId(self.0 - (self.lsb() << 1))
    }
    
    fn lsb(&self) -> u64 {
        self.0 & (!self.0 + 1)
    }
}

impl TryFrom<u64> for CellId {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(CellId(value))
    }
}
