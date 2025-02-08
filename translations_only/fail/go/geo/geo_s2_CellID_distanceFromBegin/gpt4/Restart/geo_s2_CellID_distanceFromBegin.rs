
mod test {
    const GEO_S2_MAX_LEVEL: i32 = 30;

    pub struct GeoS2CellID(u64);

    impl GeoS2CellID {
        pub fn distance_from_begin(&self) -> i64 {
            (self.0 >> (2 * (GEO_S2_MAX_LEVEL - self.level()) + 1)) as i64
        }

        pub fn level(&self) -> i32 {
            GEO_S2_MAX_LEVEL - geo_s2_find_lsb_set_non_zero64(self.0) >> 1
        }
    }

    pub fn geo_s2_find_lsb_set_non_zero64(x: u64) -> i32 {
        if x == 0 {
            0
        } else {
            x.trailing_zeros() as i32
        }
    }
}
