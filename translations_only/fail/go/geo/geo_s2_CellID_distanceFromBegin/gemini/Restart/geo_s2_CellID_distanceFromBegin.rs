
const GEO_S2_MAX_LEVEL: u8 = 30;

fn distance_from_begin(ci: u64) -> i64 {
    (ci >> (2 * (GEO_S2_MAX_LEVEL - level(ci)) + 1)) as i64
}

fn level(ci: u64) -> u8 {
    GEO_S2_MAX_LEVEL - (find_lsb_set_non_zero64(ci) as u8 >> 1)
}

fn find_lsb_set_non_zero64(x: u64) -> u32 {
    if x == 0 {
        0
    } else {
        x.trailing_zeros()
    }
}
