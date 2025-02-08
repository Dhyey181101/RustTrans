
use std::u64;

pub struct GeoS2CellID(u64);

impl GeoS2CellID {
    pub fn intersects(&self, oci: GeoS2CellID) -> bool {
        (oci.range_min().0 as u64) <= (self.range_max().0 as u64)
            && (oci.range_max().0 as u64) >= (self.range_min().0 as u64)
    }

    pub fn range_min(&self) -> GeoS2CellID {
        GeoS2CellID(self.0 - (self.lsb() - 1))
    }

    pub fn lsb(&self) -> u64 {
        self.0 & ! (self.0 - 1)
    }

    pub fn range_max(&self) -> GeoS2CellID {
        GeoS2CellID(self.0 + (self.lsb() - 1))
    }
}
