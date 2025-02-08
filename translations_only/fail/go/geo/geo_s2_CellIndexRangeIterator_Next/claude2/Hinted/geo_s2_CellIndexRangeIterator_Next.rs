
const GEO_S2_CELL_INDEX_DONE_CONTENTS: i32 = -1;

struct GeoS2CellIndexRangeIterator {
    range_nodes: Vec<GeoS2RangeNode>,
    pos: usize,
    non_empty: bool,
}

struct GeoS2RangeNode {
    contents: i32,
}

fn next(c: &mut GeoS2CellIndexRangeIterator) {
    c.pos += 1;
    while c.non_empty && is_empty(c) && !done(c) {
        c.pos += 1;
    }
}

fn done(c: &GeoS2CellIndexRangeIterator) -> bool {
    c.pos >= c.range_nodes.len() - 1
}

fn is_empty(c: &GeoS2CellIndexRangeIterator) -> bool {
    c.range_nodes[c.pos].contents == GEO_S2_CELL_INDEX_DONE_CONTENTS
}
