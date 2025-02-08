
const GEO_S2_MAX_LEVEL: u32 = 30;
const GEO_S2_POS_BITS: u32 = 2 * GEO_S2_MAX_LEVEL + 1;

fn geo_s2_cell_id_from_string(s: &str) -> u64 {
    let level = s.len() - 2;
    if level < 0 || (level as u32) > GEO_S2_MAX_LEVEL {
        return 0;
    }
    let face = (s.as_bytes()[0] - b'0') as i32;
    if face < 0 || face > 5 || s.as_bytes()[1] != b'/' {
        return 0;
    }
    let mut id = geo_s2_cell_id_from_face(face);
    for &c in &s.as_bytes()[2..] {
        let child_pos = (c - b'0') as i32;
        if child_pos < 0 || child_pos > 3 {
            return 0;
        }
        id = geo_s2_children(id)[child_pos as usize];
    }
    id
}

fn geo_s2_cell_id_from_face(face: i32) -> u64 {
    ((face as u64) << GEO_S2_POS_BITS) + geo_s2_lsb_for_level(0)
}

fn geo_s2_lsb_for_level(level: i32) -> u64 {
    1 << (2 * (GEO_S2_MAX_LEVEL - level as u32))
}

fn geo_s2_children(ci: u64) -> Box<[u64; 4]> {
    let lsb = geo_s2_lsb(ci);
    let mut ch = Box::new([0, 0, 0, 0]);
    ch[0] = ci - lsb + (lsb >> 2);
    let mut lsb = lsb >> 1;
    ch[1] = ch[0] + lsb;
    lsb >>= 1;
    ch[2] = ch[1] + lsb;
    ch[3] = ch[2] + lsb;
    ch
}

fn geo_s2_lsb(ci: u64) -> u64 {
    ci & (!ci + 1)
}
