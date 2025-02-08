
const GEO_S2_CELL_INDEX_DONE_CONTENTS: i32 = -1;

struct GeoS2RangeNode {
    contents: i32, // Contents of this node (an index within the cell tree).
}

struct GeoS2CellIndexRangeIterator {
    range_nodes: Vec<GeoS2RangeNode>,
    pos: usize,
    non_empty: bool,
}

fn geo_s2_cell_index_range_iterator_next(iterator: &mut Box<GeoS2CellIndexRangeIterator>) {
    iterator.pos += 1;
    while iterator.non_empty && geo_s2_cell_index_range_iterator_is_empty(iterator) && !geo_s2_cell_index_range_iterator_done(iterator) {
        iterator.pos += 1;
    }
}

fn geo_s2_cell_index_range_iterator_done(iterator: &GeoS2CellIndexRangeIterator) -> bool {
    iterator.pos >= iterator.range_nodes.len() - 1
}

fn geo_s2_cell_index_range_iterator_is_empty(iterator: &GeoS2CellIndexRangeIterator) -> bool {
    iterator.range_nodes[iterator.pos].contents == GEO_S2_CELL_INDEX_DONE_CONTENTS
}
