
const GEO_S2_CELL_INDEX_DONE_CONTENTS: i32 = -1;

struct GeoS2RangeNode {
    contents: i32, // Contents of this node (an index within the cell tree).
}

struct GeoS2CellIndexRangeIterator {
    range_nodes: Vec<GeoS2RangeNode>,
    pos: usize,
    non_empty: bool,
}

fn geo_s2_cell_index_range_iterator_begin(iter: &mut GeoS2CellIndexRangeIterator) {
    iter.pos = 0;
    while iter.non_empty && geo_s2_cell_index_range_iterator_is_empty(iter) && !geo_s2_cell_index_range_iterator_done(iter) {
        iter.pos += 1;
    }
}

fn geo_s2_cell_index_range_iterator_done(iter: &GeoS2CellIndexRangeIterator) -> bool {
    iter.pos >= iter.range_nodes.len() - 1
}

fn geo_s2_cell_index_range_iterator_is_empty(iter: &GeoS2CellIndexRangeIterator) -> bool {
    iter.range_nodes[iter.pos].contents == GEO_S2_CELL_INDEX_DONE_CONTENTS
}
