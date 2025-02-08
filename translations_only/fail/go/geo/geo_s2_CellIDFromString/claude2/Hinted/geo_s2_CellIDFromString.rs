
const GEO_S2_MAXLEVEL: usize = 30;
const GEO_S2_POSBITS: usize = 2*GEO_S2_MAXLEVEL + 1;

fn geo_s2_cell_id_from_string(s: &str) -> Box<GeoS2CellId> {
    let level = s.len() - 2;
    if level > GEO_S2_MAXLEVEL {
        return Box::new(GeoS2CellId(0));
    }
    
    let face = s[0..1].parse::<i32>().unwrap_or(0);
    if face < 0 || face > 5 || s.as_bytes()[1] != b'/' {
        return Box::new(GeoS2CellId(0));
    }
    
    let mut id = geo_s2_cell_id_from_face(face);
    for i in 2..s.len() {
        let child_pos = s.as_bytes()[i] - b'0';
        if child_pos < 0 || child_pos > 3 {
            return Box::new(GeoS2CellId(0));
        }
        *id = *geo_s2_get_child(&id, child_pos as u32);
    }
    id
}

fn geo_s2_cell_id_from_face(face: i32) -> Box<GeoS2CellId> {
    Box::new(GeoS2CellId(((face as u64) << GEO_S2_POSBITS) + geo_s2_lsb_for_level(0)))  
}

fn geo_s2_lsb_for_level(level: usize) -> u64 {
    1 << (2 * (GEO_S2_MAXLEVEL - level) as u64)
}

fn geo_s2_get_child(id: &Box<GeoS2CellId>, pos: u32) -> Box<GeoS2CellId> {
    Box::new(GeoS2CellId(geo_s2_get_child_id(id, pos)))
}

fn geo_s2_get_child_id(id: &Box<GeoS2CellId>, pos: u32) -> u64 {
    let lsb = geo_s2_cell_id_lsb(id);
    id.0 + (lsb >> 2) - lsb + (lsb >> (1 + pos))
}

fn geo_s2_cell_id_lsb(id: &Box<GeoS2CellId>) -> u64 {
    id.0 & (!id.0 + 1)  
}

struct GeoS2CellId(u64);

