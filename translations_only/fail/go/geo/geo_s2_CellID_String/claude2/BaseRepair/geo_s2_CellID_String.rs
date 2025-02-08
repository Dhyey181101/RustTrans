
use std::str::FromStr;
use std::fmt;
use std::num::ParseIntError;
use std::string::ToString;

const GEO_S2_NUMFACES: i32 = 6;
const GEO_S2_POSBITS: i32 = 2*GEO_S2_MAXLEVEL + 1; 
const GEO_S2_MAXLEVEL: i32 = 30;

struct GeoS2CellId(u64);

impl GeoS2CellId {
    fn is_valid(&self) -> bool {
        (self.face() < GEO_S2_NUMFACES) && ((self.lsb() & 0x1555555555555555) != 0)
    }
    
    fn face(&self) -> i32 {
        (self.0 >> GEO_S2_POSBITS) as i32
    }
    
    fn lsb(&self) -> u64 {
        self.0 & (!0u64)
    }
    
    fn child_position(&self, level: i32) -> i32 {
        ((self.0 >> (2*(GEO_S2_MAXLEVEL-level)+1) as u32) & 3) as i32
    }
    
    fn level(&self) -> i32 {
        GEO_S2_MAXLEVEL - geo_s2_find_lsb_set_non_zero64(self.0) >> 1
    }
}

impl fmt::Display for GeoS2CellId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_valid() {
            write!(f, "Invalid: {:x}", self.0)
        } else {
            let mut s = String::new();
            s.push("012345".chars().nth(self.face() as usize).unwrap());
            s.push('/');
            for level in 1..=self.level() {
                s.push("0123".chars().nth(self.child_position(level) as usize).unwrap());
            }
            write!(f, "{}", s)
        }
    }
}

impl FromStr for GeoS2CellId {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s.parse::<u64>().map(GeoS2CellId)
  }
}

fn geo_s2_find_lsb_set_non_zero64(x: u64) -> i32 {
    if x == 0 {
        0
    } else {
        x.trailing_zeros() as i32
    }
}

