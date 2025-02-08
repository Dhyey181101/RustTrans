

const GEO_S2_MAXLEVEL: u32 = 30;

fn geo_s2_are_siblings(a: CellId, b: CellId, c: CellId, d: CellId) -> bool {
    if (a ^ b ^ c) != d {
        return false;
    }
    
    let mask = lsb(d) << 1;
    let mask = !((mask + (mask << 1)) as CellId);
    let id_masked = (d as u64 & mask) as CellId;
    ((a & mask) == id_masked) 
        && ((b & mask) == id_masked)
        && ((c & mask) == id_masked)
        && !is_face(d)
}

fn lsb(ci: CellId) -> u64 {
    ci as u64 & -(ci as i64) as u64
}

fn is_face(ci: CellId) -> bool {
    (ci as u64) & (geo_s2_lsb_for_level(0) - 1) == 0 
}

fn geo_s2_lsb_for_level(level: u32) -> u64 {
    1 << (2 * (GEO_S2_MAXLEVEL - level) as u32)
}

type CellId = u64;

