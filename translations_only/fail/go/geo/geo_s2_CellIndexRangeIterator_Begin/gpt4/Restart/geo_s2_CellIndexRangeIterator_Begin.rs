
const GEO_S2_CELL_INDEX_DONE_CONTENTS: i32 = -1;

struct GeoS2RangeNode {
    contents: i32,
}

struct GeoS2CellIndexRangeIterator {
    range_nodes: Vec<GeoS2RangeNode>,
    pos: usize,
    non_empty: bool,
}

impl GeoS2CellIndexRangeIterator {
    fn new(range_nodes: Vec<GeoS2RangeNode>, non_empty: bool) -> Box<Self> {
        Box::new(Self {
            range_nodes,
            pos: 0,
            non_empty,
        })
    }
}

fn begin(iter: &mut GeoS2CellIndexRangeIterator) {
    iter.pos = 0;
    while iter.non_empty && is_empty(iter) && !done(iter) {
        iter.pos += 1;
    }
}

fn done(iter: &GeoS2CellIndexRangeIterator) -> bool {
    iter.pos >= iter.range_nodes.len() - 1
}

fn is_empty(iter: &GeoS2CellIndexRangeIterator) -> bool {
    iter.range_nodes[iter.pos].contents == GEO_S2_CELL_INDEX_DONE_CONTENTS
}
