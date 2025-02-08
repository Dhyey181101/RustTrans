

const NUM_FACES: u32 = 6;
const MAX_LEVEL: u32 = 30;
const POS_BITS: u32 = 2 * MAX_LEVEL + 1;
const WRAP_OFFSET: u64 = (NUM_FACES as u64) << POS_BITS;

type CellID = u64;

fn prev_wrap(ci: CellID) -> CellID {
    let p = ci.wrapping_sub(get_lsb(ci) << 1);
    if p < WRAP_OFFSET {
        return p;
    }
    return p.wrapping_add(WRAP_OFFSET);
}

fn prev(ci: CellID) -> CellID {
    ci.wrapping_sub(get_lsb(ci) << 1)
}

fn get_lsb(ci: CellID) -> u64 {
    ci & ! (ci - 1)
}

