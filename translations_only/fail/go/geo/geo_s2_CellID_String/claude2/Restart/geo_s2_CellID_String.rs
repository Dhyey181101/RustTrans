
use std::str::FromStr;
use std::fmt;
use std::num::ParseIntError;
use std::convert::TryFrom;

const GEO_S2_NUMFACES: i32 = 6;
const GEO_S2_POSBITS: i32 = 2*GEO_S2_MAXLEVEL + 1;
const GEO_S2_MAXLEVEL: i32 = 30;

struct GeoS2CellId(u64);

impl fmt::Display for GeoS2CellId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !is_valid(self.0) {
            return write!(f, "Invalid: {:x}", self.0);
        }
        
        let mut s = "012345".chars().nth(face(self.0) as usize).unwrap().to_string();
        s.push('/');
        for level in 1..=level(self.0) {
            s.push(char::from_digit(child_position(self.0, level), 10).unwrap());
        }

        write!(f, "{}", s)
    }
}

fn is_valid(id: u64) -> bool {
    face(id) < GEO_S2_NUMFACES && (lsb(id) & 0x1555555555555555) != 0
}

fn face(id: u64) -> i32 {
    (id >> GEO_S2_POSBITS) as i32
}

fn lsb(id: u64) -> u64 {
    id & (!id + 1)  
}

fn child_position(id: u64, level: i32) -> u32 {
    ((id >> (2*(GEO_S2_MAXLEVEL - level) + 1)) & 3) as u32
}

fn level(id: u64) -> i32 {
    GEO_S2_MAXLEVEL - find_lsb_set_non_zero64(id) >> 1
}

fn find_lsb_set_non_zero64(x: u64) -> i32 {
    if x == 0 { 
        0
    } else {
        x.trailing_zeros() as i32
    }
}

impl TryFrom<u64> for GeoS2CellId {
    type Error = ParseIntError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(GeoS2CellId(value))
    }
}

