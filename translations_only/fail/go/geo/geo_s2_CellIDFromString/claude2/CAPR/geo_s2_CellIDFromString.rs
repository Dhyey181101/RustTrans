

const GEO_S2_MAXLEVEL: usize = 30;
const GEO_S2_POSBITS: i32 = 2*GEO_S2_MAXLEVEL as i32 + 1;

#[derive(Copy, Clone)]
struct CellId(u64);

fn geo_s2_cell_id_from_string(s: &str) -> CellId {
    let level = s.len() - 2;
    if level < 0 || level as usize > GEO_S2_MAXLEVEL {
        return CellId(0);
    }
    
    let face = s[0..1].parse::<i32>().unwrap() - 48;
    if face < 0 || face > 5 || *s.get(1..2).unwrap() != *"/" {
        return CellId(0);
    }
    
    let mut id = geo_s2_cell_id_from_face(face);
    let mut i = 2;
    while i < s.len() {
        let child_pos = s[i..i+1].parse::<i32>().unwrap() - 48;
        if child_pos < 0 || child_pos > 3 {
            return CellId(0);
        }
        id = geo_s2_children(&id)[child_pos as usize];
        i += 1;
    }
    id
}

fn geo_s2_cell_id_from_face(face: i32) -> CellId {
    CellId(((face as u64) << GEO_S2_POSBITS) + geo_s2_lsb_for_level(0))  
}

fn geo_s2_lsb_for_level(level: i32) -> u64 {
    1 << (2*(GEO_S2_MAXLEVEL as i32-level) as u64)
}

fn geo_s2_children(id: &CellId) -> Box<[CellId; 4]> {
    let repeat_value = CellId(0);
    let mut ch = Box::new([repeat_value; 4]);
    let lsb = geo_s2_lsb(&id);
        
    ch[0] = CellId(id.0 - lsb + (lsb >> 2));
    let lsb = lsb >> 1;
        
    ch[1] = CellId(ch[0].0 + lsb);
    ch[2] = CellId(ch[1].0 + lsb);
    ch[3] = CellId(ch[2].0 + lsb);
        
    ch
}

fn geo_s2_lsb(id: &CellId) -> u64 {
    id.0 & (!id.0 + 1)
}

