
const GEO_S2_NUM_FACES: usize = 6;
const GEO_S2_POS_BITS: usize = 2 * GEO_S2_MAX_LEVEL + 1;
const GEO_S2_MAX_LEVEL: usize = 30;

#[derive(Copy, Clone, Debug)]
pub struct GeoS2CellID(u64);

impl GeoS2CellID {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn is_valid(&self) -> bool {
        self.0 < (GEO_S2_NUM_FACES as u64) << GEO_S2_POS_BITS
            && (self.0 & 0x1555555555555555 != 0)
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
        GEO_S2_MAX_LEVEL - geo_s2_find_lsb_set_non_zero_64(self.0) >> 1
    }

    pub fn to_string(&self) -> String {
        if !self.is_valid() {
            return format!("Invalid: {:x}", self.0);
        }

        let mut buf = String::new();
        buf.push_str(&"012345"[self.face()..self.face() + 1]);
        buf.push('/');
        for level in 1..=self.level() {
            buf.push_str(&"0123"[self.child_position(level)..self.child_position(level) + 1]);
        }
        buf
    }
}

fn geo_s2_find_lsb_set_non_zero_64(x: u64) -> usize {
    if x == 0 {
        return 0;
    }
    x.trailing_zeros() as usize
}
