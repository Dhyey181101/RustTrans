
use std::u64;

pub struct GeoS2CellID(u64);

impl GeoS2CellID {
    pub fn intersects(&self, oci: GeoS2CellID) -> bool {
        (oci.0 & !(oci.0 - 1)) <= (self.0 & !(self.0 - 1)) && (oci.0 | (oci.0 - 1)) >= (self.0 | (self.0 - 1))
    }

    pub fn range_min(&self) -> GeoS2CellID {
        GeoS2CellID(self.0.saturating_sub(self.lsb() - 1))
    }

    pub fn lsb(&self) -> u64 {
        self.0 & ! (self.0 - 1)
    }

    pub fn range_max(&self) -> GeoS2CellID {
        GeoS2CellID(self.0.saturating_add(self.lsb() - 1))
    }
}

fn main() {
    let test_cases = [
        (134250752, 2885118511296000),
        (16360139164282316554, 16357074000756859875),
        (84974535979318794, 0),
        (216172893786996746, 0),
    ];

    for (ci, oci) in test_cases {
        let c = GeoS2CellID(ci);
        let o = GeoS2CellID(oci);
        println!("Intersects: {} and {} = {}", ci, oci, c.intersects(o));
    }
}
