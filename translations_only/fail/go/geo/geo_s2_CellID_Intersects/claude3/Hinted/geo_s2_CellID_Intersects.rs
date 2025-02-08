

use std::u64;

#[derive(Copy, Clone)]
struct GeoS2CellID(u64);

fn range_min(ci: GeoS2CellID) -> u64 {
    let lsb = lsb(ci.0);
    (ci.0 - (lsb - 1)) as u64
}

fn lsb(value: u64) -> u64 {
    value & (!value + 1)
}

fn range_max(ci: GeoS2CellID) -> u64 {
    let lsb = lsb(ci.0);
    (ci.0 + (lsb - 1)) as u64
}

fn intersects(ci: GeoS2CellID, oci: GeoS2CellID) -> bool {
    range_min(oci) <= range_max(ci) && range_max(oci) >= range_min(ci)
}

fn main() {
    let test_cases = [
        (GeoS2CellID(0), GeoS2CellID(281474976774400)),
        (GeoS2CellID(10475047277821962), GeoS2CellID(0)),
        (GeoS2CellID(18158513697557844224), GeoS2CellID(864691132750102527)),
        (GeoS2CellID(16640), GeoS2CellID(9008462773989933057)),
    ];

    for (ci, oci) in test_cases.iter() {
        let result = intersects(*ci, *oci);
        println!("Input is invalid, crash gracefully");
    }
}

