
const GEO_S2_MAX_LEVEL: u8 = 30;
const GEO_S2_POS_BITS: u8 = 2 * GEO_S2_MAX_LEVEL + 1;

pub fn geo_s2_cell_id_from_string(s: &str) -> Option<u64> {
    let level = s.len() - 2;
    if level < 0 || level > GEO_S2_MAX_LEVEL as usize {
        return None;
    }
    let face = s.chars().nth(0).unwrap().to_digit(10).unwrap() as u8;
    if face < 0 || face > 5 || s.chars().nth(1).unwrap() != '/' {
        return None;
    }
    let mut id = geo_s2_cell_id_from_face(face);
    for i in 2..s.len() {
        let child_pos = s.chars().nth(i).unwrap().to_digit(10).unwrap() as u8;
        if child_pos < 0 || child_pos > 3 {
            return None;
        }
        id = children(id)[child_pos as usize];
    }
    Some(id)
}

pub fn geo_s2_cell_id_from_face(face: u8) -> u64 {
    (face as u64) << GEO_S2_POS_BITS | geo_s2_lsb_for_level(0)
}

pub fn geo_s2_lsb_for_level(level: u8) -> u64 {
    1 << (2 * (GEO_S2_MAX_LEVEL - level) as u64)
}

pub fn children(ci: u64) -> [u64; 4] {
    let mut ch = [0; 4];
    let lsb = lsb(ci);
    ch[0] = ci - lsb + lsb >> 2;
    let lsb = lsb >> 1;
    ch[1] = ch[0] + lsb;
    ch[2] = ch[1] + lsb;
    ch[3] = ch[2] + lsb;
    ch
}

pub fn lsb(ci: u64) -> u64 {
    ci & (ci.wrapping_neg() & ci)
}
