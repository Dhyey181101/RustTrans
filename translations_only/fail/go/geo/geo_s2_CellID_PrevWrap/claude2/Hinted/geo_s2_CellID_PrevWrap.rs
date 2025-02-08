
const GEO_S2_NUMFACES: u32 = 6;
const GEO_S2_MAXLEVEL: u32 = 30;
const GEO_S2_POSBITS: u32 = 2 * GEO_S2_MAXLEVEL + 1;
const GEO_S2_WRAPOFFSET: u64 = (GEO_S2_NUMFACES as u64) << GEO_S2_POSBITS;

struct GeoS2CellId(u64);

fn prev_wrap(ci: GeoS2CellId) -> GeoS2CellId {
    let p = prev(ci);
    if p.0 < GEO_S2_WRAPOFFSET {
        p
    } else {
        GeoS2CellId(p.0 + GEO_S2_WRAPOFFSET)
    }
}

fn prev(ci: GeoS2CellId) -> GeoS2CellId {
    GeoS2CellId(ci.0 - lsb(&ci) << 1)  
}

fn lsb(ci: &GeoS2CellId) -> u64 {
    ci.0 & (!ci.0 + 1)
}
