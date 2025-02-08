
use std::fmt;

const GEO_S2_NUM_FACES: i32 = 6;
const GEO_S2_POS_BITS: i32 = 2 * GEO_S2_MAX_LEVEL + 1;
const GEO_S2_MAX_LEVEL: i32 = 30;

#[derive(Copy, Clone)]
struct GeoS2CellID(u64);

impl fmt::Display for GeoS2CellID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !is_valid(*self) {
            return write!(f, "Invalid: {:x}", self.0);
        }
        let mut result = String::from("012345".chars().nth(face(*self) as usize).unwrap());
        result.push('/');
        for level in 1..=level(*self) {
            result.push("0123".chars().nth(child_position(*self, level) as usize).unwrap());
        }
        write!(f, "{}", result)
    }
}

fn is_valid(ci: GeoS2CellID) -> bool {
    face(ci) < GEO_S2_NUM_FACES && (lsb(ci) & 0x1555555555555555 != 0)
}

fn face(ci: GeoS2CellID) -> i32 {
    (ci.0 >> GEO_S2_POS_BITS) as i32
}

fn lsb(ci: GeoS2CellID) -> u64 {
    ci.0 & ci.0.wrapping_neg()
}

fn child_position(ci: GeoS2CellID, level: i32) -> i32 {
    ((ci.0 >> (2 * (GEO_S2_MAX_LEVEL - level) + 1)) & 3) as i32
}

fn level(ci: GeoS2CellID) -> i32 {
    GEO_S2_MAX_LEVEL - (find_lsb_set_non_zero_64(ci.0) >> 1)
}

fn find_lsb_set_non_zero_64(x: u64) -> i32 {
    if x == 0 {
        0
    } else {
        x.trailing_zeros() as i32
    }
}

fn main() {}
