
use std::fmt;

const GEO_S2_NUM_FACES: usize = 6;
const GEO_S2_POS_BITS: usize = 2 * GEO_S2_MAX_LEVEL + 1;
const GEO_S2_MAX_LEVEL: usize = 30;

#[derive(Copy, Clone)]
pub struct GeoS2CellID(u64);

impl fmt::Display for GeoS2CellID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_valid() {
            write!(f, "Invalid: {:x}", self.0)
        } else {
            write!(f, "012345[{}]/", self.face())?;
            for level in 1..=self.level() {
                write!(f, "0123[{}]", self.child_position(level))?;
            }
            Ok(())
        }
    }
}

impl GeoS2CellID {
    pub fn is_valid(&self) -> bool {
        self.face() < GEO_S2_NUM_FACES && (self.lsb() & 0x1555555555555555 != 0)
    }

    pub fn face(&self) -> usize {
        (self.0 >> GEO_S2_POS_BITS) as usize
    }

    pub fn lsb(&self) -> u64 {
        self.0 & !self.0
    }

    pub fn child_position(&self, level: usize) -> usize {
        ((self.0 >> (2 * (GEO_S2_MAX_LEVEL - level) + 1)) & 3) as usize
    }

    pub fn level(&self) -> usize {
        GEO_S2_MAX_LEVEL - geo_s2_find_lsb_set_non_zero64(self.0) >> 1
    }
}

fn geo_s2_find_lsb_set_non_zero64(x: u64) -> usize {
    if x == 0 {
        0
    } else {
        x.trailing_zeros() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input() {
        let invalid_inputs = [
            19141269078933504,
            721209259076878336,
            780575560841976918,
            7556029570674616936,
        ];
        for input in invalid_inputs {
            assert!(!GeoS2CellID(input).is_valid());
        }
    }
}
