
struct GeoS2CellID(u64);

impl GeoS2CellID {
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

fn intersects(ci: GeoS2CellID, oci: GeoS2CellID) -> bool {
    oci.range_min().0 <= ci.range_max().0 && oci.range_max().0 >= ci.range_min().0
}

fn main() {
    println!("{}", intersects(GeoS2CellID(150119978631168), GeoS2CellID(1152921504606846976))); // True
    println!("{}", intersects(GeoS2CellID(45240322085), GeoS2CellID(1152921504606846976))); // True
    println!("{}", intersects(GeoS2CellID(176160767), GeoS2CellID(6078602243471310848))); // False
    println!("{}", intersects(GeoS2CellID(9871890383196127231), GeoS2CellID(18400544281028212557))); // False
}
