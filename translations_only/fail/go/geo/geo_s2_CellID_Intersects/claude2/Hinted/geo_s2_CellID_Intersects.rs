
use std::convert::TryInto;

fn intersects(ci: CellId, oci: CellId) -> bool {
    range_min(oci) <= range_max(ci) && range_max(oci) >= range_min(ci)
}

fn range_min(ci: CellId) -> CellId {
    ci - (lsb(ci) - 1)
}

fn lsb(ci: CellId) -> CellId {
    ci & (!ci + 1)
} 

fn range_max(ci: CellId) -> CellId {
    ci + (lsb(ci) - 1)  
}

type CellId = u64;
