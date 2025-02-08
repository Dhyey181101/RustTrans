
const GEO_S2_MAX_LEVEL: i32 = 30;
const GEO_S2_POS_BITS: u64 = 2 * GEO_S2_MAX_LEVEL as u64 + 1;

#[derive(Copy, Clone)]
struct GeoS2CellID(u64);

fn children(id: GeoS2CellID) -> [GeoS2CellID; 4] {
    let mut ch = [GeoS2CellID(0); 4];
    let lsb = lsb(id);
    ch[0] = GeoS2CellID(id.0 - lsb + (lsb >> 2));
    let lsb = lsb >> 1;
    ch[1] = GeoS2CellID(ch[0].0 + lsb);
    ch[2] = GeoS2CellID(ch[1].0 + lsb);
    ch[3] = GeoS2CellID(ch[2].0 + lsb);
    ch
}

fn lsb(id: GeoS2CellID) -> u64 {
    id.0 & (!id.0 + 1)
}

fn geo_s2_cell_id_from_string(s: &str) -> GeoS2CellID {
    let level = s.len() as i32 - 2;
    if level < 0 || level > GEO_S2_MAX_LEVEL {
        return GeoS2CellID(0);
    }
    let face = s.chars().nth(0).unwrap() as i32 - '0' as i32;
    if face < 0 || face > 5 || s.chars().nth(1).unwrap() != '/' {
        return GeoS2CellID(0);
    }
    let mut id = geo_s2_cell_id_from_face(face);
    for c in s.chars().skip(2) {
        let child_pos = c as i32 - '0' as i32;
        if child_pos < 0 || child_pos > 3 {
            return GeoS2CellID(0);
        }
        let children = children(id);
        id = children[child_pos as usize];
    }
    id
}

fn geo_s2_cell_id_from_face(face: i32) -> GeoS2CellID {
    GeoS2CellID(((face as u64) << GEO_S2_POS_BITS) + geo_s2_lsb_for_level(0))
}

fn geo_s2_lsb_for_level(level: i32) -> u64 {
    1 << (2 * (GEO_S2_MAX_LEVEL - level)) as u64
}
