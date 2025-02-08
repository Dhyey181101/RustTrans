
const GEO_S2_NUM_FACES: u8 = 6;
const GEO_S2_POS_BITS: u8 = 2 * GEO_S2_MAX_LEVEL + 1;
const GEO_S2_MAX_LEVEL: u8 = 30;

fn geo_s2_find_lsb_set_non_zero64(x: u64) -> u8 {
    if x == 0 {
        return 0;
    }
    x.trailing_zeros() as u8
}

#[derive(Debug, Copy, Clone)]
pub struct GeoS2CellID(pub u64);

impl GeoS2CellID {
    pub fn is_valid(&self) -> bool {
        self.0 < (GEO_S2_NUM_FACES as u64) << GEO_S2_POS_BITS
            && (self.lsb() & 0x1555555555555555 != 0)
    }

    pub fn face(&self) -> u8 {
        (self.0 >> GEO_S2_POS_BITS) as u8
    }

    pub fn lsb(&self) -> u64 {
        self.0 & self.0.reverse_bits()
    }

    pub fn child_position(&self, level: u8) -> u8 {
        ((self.0 >> (2 * (GEO_S2_MAX_LEVEL - level) + 1)) & 3) as u8
    }

    pub fn level(&self) -> u8 {
        GEO_S2_MAX_LEVEL - geo_s2_find_lsb_set_non_zero64(self.0) >> 1
    }

    pub fn to_string(&self) -> String {
        if !self.is_valid() {
            format!("Invalid: {:x}", self.0)
        } else {
            let mut buf = String::new();
            buf.push_str(&"012345"[self.face() as usize..self.face() as usize + 1]);
            buf.push('/');
            for level in 1..=self.level() {
                buf.push_str(&"0123"[self.child_position(level) as usize..self.child_position(level) as usize + 1]);
            }
            buf
        }
    }
}
