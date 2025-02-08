
use std::ops::Range;

struct CellID(u64);

fn intersects(ci: &CellID, oci: &CellID) -> bool {
    let ci_min = range_min(ci);
    let ci_max = range_max(ci);
    let oci_min = range_min(oci);
    let oci_max = range_max(oci);

    oci_min.0 <= ci_max.0 && oci_max.0 >= ci_min.0
}

fn range_min(ci: &CellID) -> CellID {
    CellID(ci.0 - (lsb(ci) - 1))  
}

fn lsb(ci: &CellID) -> u64 {
    ci.0 & (!ci.0 + 1)
}

fn range_max(ci: &CellID) -> CellID {
    CellID(ci.0 + (lsb(ci) - 1))
}

