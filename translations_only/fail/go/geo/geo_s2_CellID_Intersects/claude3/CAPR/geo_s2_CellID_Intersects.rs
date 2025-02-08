
use std::ops::{BitAnd, BitOr, Not, Shl, Shr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct GeoS2CellID(u64);

impl GeoS2CellID {
    fn intersects(&self, oci: GeoS2CellID) -> bool {
        (oci.range_min().0 as u64) <= self.range_max().0
            && (oci.range_max().0 as u64) >= self.range_min().0
    }

    fn range_min(&self) -> GeoS2CellID {
        GeoS2CellID((self.0 - (self.lsb() - 1)) as u64)
    }

    fn lsb(&self) -> u64 {
        (self.0 & (!self.0).shr(63)) as u64
    }

    fn range_max(&self) -> GeoS2CellID {
        GeoS2CellID((self.0 + (self.lsb() - 1)) as u64)
    }
}
