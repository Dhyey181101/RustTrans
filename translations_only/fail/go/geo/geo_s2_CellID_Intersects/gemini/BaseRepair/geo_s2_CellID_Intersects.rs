
pub fn intersects(ci: u64, oci: u64) -> bool {
    range_min(oci) <= range_max(ci) && range_max(oci) >= range_min(ci)
}

pub fn range_min(ci: u64) -> u64 {
    ci - (ci.trailing_zeros() as u64)
}

pub fn lsb(ci: u64) -> u64 {
    ci & ci.wrapping_neg()
}

pub fn range_max(ci: u64) -> u64 {
    ci + (ci.trailing_zeros() as u64) - 1
}
