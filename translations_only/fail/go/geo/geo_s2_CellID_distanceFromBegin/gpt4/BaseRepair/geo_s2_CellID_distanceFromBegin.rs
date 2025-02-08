
mod test {
    const GEO_S2_MAX_LEVEL: i32 = 30;

    pub fn distance_from_begin(ci: u64) -> i64 {
        (ci >> (2 * (GEO_S2_MAX_LEVEL - level(ci)) + 1)) as i64
    }

    pub fn level(ci: u64) -> i32 {
        GEO_S2_MAX_LEVEL - (find_lsb_set_non_zero64(ci) >> 1)
    }

    fn find_lsb_set_non_zero64(x: u64) -> i32 {
        if x == 0 {
            0
        } else {
            x.trailing_zeros() as i32
        }
    }
}
