
const GEO_S2_MAX_LEVEL: usize = 30;
const GEO_S2_POS_BITS: u64 = 2 * GEO_S2_MAX_LEVEL as u64 + 1;

#[derive(Clone, Copy)]
struct GeoS2CellID(u64);

fn geo_s2_cell_id_from_string(s: &str) -> GeoS2CellID {
    let level = s.len() - 2;
    if level < 0 || level > GEO_S2_MAX_LEVEL {
        return GeoS2CellID(0);
    }
    let face = s.chars().nth(0).unwrap() as i32 - '0' as i32;
    if face < 0 || face > 5 || s.chars().nth(1).unwrap() != '/' {
        return GeoS2CellID(0);
    }
    let mut id = geo_s2_cell_id_from_face(face as usize);
    for c in s[2..].chars() {
        let child_pos = c as i32 - '0' as i32;
        if child_pos < 0 || child_pos > 3 {
            return GeoS2CellID(0);
        }
        id = children(&id)[child_pos as usize];
    }
    id
}

fn geo_s2_cell_id_from_face(face: usize) -> GeoS2CellID {
    GeoS2CellID((face as u64) << GEO_S2_POS_BITS | geo_s2_lsb_for_level(0))
}

fn geo_s2_lsb_for_level(level: usize) -> u64 {
    1 << (2 * (GEO_S2_MAX_LEVEL - level)) as u64
}

fn children(ci: &GeoS2CellID) -> [GeoS2CellID; 4] {
    let mut ch = [GeoS2CellID(0); 4];
    let lsb = lsb(ci);
    ch[0] = GeoS2CellID(ci.0 - lsb + (lsb >> 2));
    let lsb = lsb >> 1;
    ch[1] = GeoS2CellID(ch[0].0 + lsb);
    ch[2] = GeoS2CellID(ch[1].0 + lsb);
    ch[3] = GeoS2CellID(ch[2].0 + lsb);
    ch
}

fn lsb(ci: &GeoS2CellID) -> u64 {
    ci.0 & ci.0.wrapping_neg()
}

fn main() {
    println!("{:?}", geo_s2_cell_id_from_string("1[").0);
    println!("{:?}", geo_s2_cell_id_from_string("/").0);
    println!("{:?}", geo_s2_cell_id_from_string("3/0123").0);
    println!("{:?}", geo_s2_cell_id_from_string("4/2023").0);
}
