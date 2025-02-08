
const GEO_S2_NUM_FACES: u64 = 6;
const GEO_S2_MAX_LEVEL: u64 = 30;
const GEO_S2_POS_BITS: u64 = 2 * GEO_S2_MAX_LEVEL + 1;
const GEO_S2_WRAP_OFFSET: u64 = GEO_S2_NUM_FACES << GEO_S2_POS_BITS;

fn prev_wrap(ci: u64) -> u64 {
    let p = prev(ci);
    if p < GEO_S2_WRAP_OFFSET {
        p
    } else {
        p + GEO_S2_WRAP_OFFSET
    }
}

fn prev(ci: u64) -> u64 {
    ci & !(ci - 1)
}
