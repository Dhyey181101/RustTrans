
const NUM_FACES: u32 = 6;
const MAX_LEVEL: u32 = 30;
const POS_BITS: u32 = 2 * MAX_LEVEL + 1;
const WRAP_OFFSET: u64 = (NUM_FACES as u64) << POS_BITS;

fn prev_wrap(ci: CellID) -> CellID {
    let p = prev(ci);
    if p.0 < WRAP_OFFSET {
        return p;
    }
    CellID(p.0 + WRAP_OFFSET)
}

fn prev(ci: CellID) -> CellID {
    let lsb = ci.0 & ! (ci.0 - 1);
    CellID(ci.0 - lsb << 1)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct CellID(u64);
