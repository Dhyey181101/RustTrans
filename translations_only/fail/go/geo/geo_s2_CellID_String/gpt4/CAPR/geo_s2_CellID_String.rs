
use std::fmt;

const GEO_S2_NUM_FACES: i32 = 6;
const GEO_S2_POS_BITS: u64 = 2 * GEO_S2_MAX_LEVEL as u64 + 1;
const GEO_S2_MAX_LEVEL: i32 = 30;

struct GeoS2CellID(u64);

impl GeoS2CellID {
    fn is_valid(&self) -> bool {
        self.face() < GEO_S2_NUM_FACES && (self.lsb() & 0x1555555555555555 != 0)
    }

    fn face(&self) -> i32 {
        (self.0 >> GEO_S2_POS_BITS) as i32
    }

    fn lsb(&self) -> u64 {
        self.0 & self.0.wrapping_neg()
    }

    fn child_position(&self, level: i32) -> i32 {
        ((self.0 >> (2 * (GEO_S2_MAX_LEVEL - level) + 1)) & 3) as i32
    }

    fn level(&self) -> i32 {
        GEO_S2_MAX_LEVEL - (geo_s2_find_lsb_set_non_zero64(self.0) >> 1) as i32
    }
}

fn geo_s2_find_lsb_set_non_zero64(x: u64) -> u32 {
    x.trailing_zeros()
}

impl fmt::Display for GeoS2CellID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_valid() {
            return write!(f, "Invalid: {:x}", self.0);
        }
        let mut result = String::from("012345".chars().nth(self.face() as usize).unwrap());
        result.push('/');
        for level in 1..=self.level() {
            result.push("0123".chars().nth(self.child_position(level) as usize).unwrap());
        }
        write!(f, "{}", result)
    }
}
