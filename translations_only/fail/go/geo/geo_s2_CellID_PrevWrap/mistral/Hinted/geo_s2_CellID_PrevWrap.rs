

const GEO_S2_NUM_FACES: u32 = 6;
const GEO_S2_MAX_LEVEL: u32 = 30;
const GEO_S2_POS_BITS: u32 = 2 * GEO_S2_MAX_LEVEL + 1;
const GEO_S2_WRAP_OFFSET: u64 = (1 << GEO_S2_POS_BITS as u64) * GEO_S2_NUM_FACES as u64;

