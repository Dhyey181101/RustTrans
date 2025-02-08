
#[derive(Copy, Clone)]
struct GeoS2CellID(u64);

impl GeoS2CellID {
    fn intersects(&self, oci: GeoS2CellID) -> bool {
        (oci.range_min().0 <= self.range_max().0) && (oci.range_max().0 >= self.range_min().0)
    }

    fn range_min(&self) -> GeoS2CellID {
        GeoS2CellID(self.0 - (self.lsb() - 1))
    }

    fn lsb(&self) -> u64 {
        self.0 & (!self.0 + 1)
    }

    fn range_max(&self) -> GeoS2CellID {
        GeoS2CellID(self.0 + (self.lsb() - 1))
    }
}
