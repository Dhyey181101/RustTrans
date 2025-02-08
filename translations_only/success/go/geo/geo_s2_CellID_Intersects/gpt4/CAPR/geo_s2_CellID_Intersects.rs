
struct GeoS2CellID(u64);

impl GeoS2CellID {
    fn lsb(&self) -> u64 {
        self.0 & (!self.0 + 1)
    }

    fn range_min(&self) -> GeoS2CellID {
        GeoS2CellID(self.0 - (self.lsb() - 1))
    }

    fn range_max(&self) -> GeoS2CellID {
        GeoS2CellID(self.0 + (self.lsb() - 1))
    }
}

fn intersects(ci: &GeoS2CellID, oci: &GeoS2CellID) -> bool {
    oci.range_min().0 <= ci.range_max().0 && oci.range_max().0 >= ci.range_min().0
}
