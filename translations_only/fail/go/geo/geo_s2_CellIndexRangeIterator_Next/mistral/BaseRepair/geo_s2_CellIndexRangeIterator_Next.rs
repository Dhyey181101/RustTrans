

use std::boxed::Box;

const GEO_S2_CELLINDEXDONECONTENTS: i32 = -1;

struct GeoS2RangeNode {
    contents: i32,
}

struct GeoS2CellIndexRangeIterator {
    range_nodes: Vec<GeoS2RangeNode>,
    pos: usize,
    non_empty: bool,
}

impl GeoS2CellIndexRangeIterator {
    fn new(range_nodes: Vec<GeoS2RangeNode>) -> Box<Self> {
        let non_empty = !range_nodes.is_empty();
        Box::new(GeoS2CellIndexRangeIterator {
            range_nodes,
            pos: 0,
            non_empty,
        })
    }

    fn next(&mut self) {
        self.pos += 1;
        while self.non_empty && self.is_empty() && !self.done() {
            self.pos += 1;
        }
    }

    fn done(&self) -> bool {
        self.pos >= self.range_nodes.len() - 1
    }

    fn is_empty(&self) -> bool {
        self.range_nodes[self.pos].contents == GEO_S2_CELLINDEXDONECONTENTS
    }
}

