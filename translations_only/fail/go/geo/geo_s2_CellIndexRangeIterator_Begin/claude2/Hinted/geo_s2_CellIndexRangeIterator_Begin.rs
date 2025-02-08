
const GEO_S2_CELL_INDEX_DONE_CONTENTS: i32 = -1;

struct GeoS2CellIndexRangeIterator {
    range_nodes: Vec<GeoS2RangeNode>,
    pos: usize,
    non_empty: bool,
}

struct GeoS2RangeNode {
    contents: i32,
}

fn geo_s2_cell_index_range_iterator_begin(c: &mut GeoS2CellIndexRangeIterator) {
    c.pos = 0;
    while c.non_empty && geo_s2_cell_index_range_iterator_is_empty(c) && !geo_s2_cell_index_range_iterator_done(c) {
        c.pos += 1;
    }
}

fn geo_s2_cell_index_range_iterator_done(c: &GeoS2CellIndexRangeIterator) -> bool {
    c.pos >= c.range_nodes.len() - 1
}

fn geo_s2_cell_index_range_iterator_is_empty(c: &GeoS2CellIndexRangeIterator) -> bool {
    c.range_nodes[c.pos].contents == GEO_S2_CELL_INDEX_DONE_CONTENTS
}
