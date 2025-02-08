

const MAX_LEVEL: u32 = 30;

fn geo_s2_are_siblings(a: u64, b: u64, c: u64, d: u64) -> bool {
    if a ^ b ^ c != d {
        return false;
    }
    
    let mask = lsb(d) << 1;
    let mask = !((mask + (mask << 1)) as u64);
    let id_masked = d & mask;
    ((a & mask) == id_masked) 
        && ((b & mask) == id_masked)
        && ((c & mask) == id_masked)
        && !is_face(d)
}

fn lsb(ci: u64) -> u64 {
    ci & (!ci + 1)
}

fn is_face(ci: u64) -> bool {
    (ci & (lsb_for_level(0) - 1)) == 0 
}

fn lsb_for_level(level: u32) -> u64 {
    1 << (2 * (MAX_LEVEL - level) as u64)  
}

