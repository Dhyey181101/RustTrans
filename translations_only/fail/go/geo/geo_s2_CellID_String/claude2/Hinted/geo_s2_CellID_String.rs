
use std::str::FromStr;
use std::fmt;
use std::num::ParseIntError;

const GEO_S2_NUMFACES: i32 = 6;
const GEO_S2_POSBITS: i32 = 2*GEO_S2_MAXLEVEL + 1;
const GEO_S2_MAXLEVEL: i32 = 30;

struct GeoS2CellId(u64);

fn is_valid(cell_id: &GeoS2CellId) -> bool {
    (face(cell_id) < GEO_S2_NUMFACES) && ((lsb(cell_id) & 0x1555555555555555) != 0)
}

fn face(cell_id: &GeoS2CellId) -> i32 {
    (cell_id.0 >> GEO_S2_POSBITS) as i32
}

fn lsb(cell_id: &GeoS2CellId) -> u64 {
    cell_id.0 & (!0 - cell_id.0)  
}

fn child_position(cell_id: &GeoS2CellId, level: i32) -> i32 {
    ((cell_id.0 >> (2*(GEO_S2_MAXLEVEL - level) + 1)) & 3) as i32
}

fn level(cell_id: &GeoS2CellId) -> i32 {
    GEO_S2_MAXLEVEL - geo_s2_find_lsb_set_non_zero64(cell_id.0) >> 1
}

fn to_string(cell_id: &GeoS2CellId) -> String {
    if !is_valid(cell_id) {
         return "Invalid: ".to_owned() + &cell_id.0.to_string();
    }
    let mut result = "012345".chars().nth(face(cell_id) as usize).unwrap().to_string();
    result.push('/');
    for level in 1..=level(cell_id) {
        result.push("0123".chars().nth(child_position(cell_id, level) as usize).unwrap());
    }
    result
}

impl fmt::Display for GeoS2CellId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", to_string(self))
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

