
struct GeoS2RangeNode {
    contents: i32,
}

struct GeoS2CellIndexRangeIterator {
    range_nodes: Vec<GeoS2RangeNode>,
    pos: usize,
    non_empty: bool,
}

const GEO_S2_CELL_INDEX_DONE_CONTENTS: i32 = -1;

impl GeoS2CellIndexRangeIterator {
    fn new(range_nodes: Vec<GeoS2RangeNode>, non_empty: bool) -> Box<Self> {
        Box::new(Self {
            range_nodes,
            pos: 0,
            non_empty,
        })
    }

    fn begin(&mut self) {
        self.pos = 0;
        while self.non_empty && self.is_empty() && !self.done() {
            self.pos += 1;
        }
    }

    fn done(&self) -> bool {
        self.pos >= self.range_nodes.len() - 1
    }

    fn is_empty(&self) -> bool {
        self.range_nodes[self.pos].contents == GEO_S2_CELL_INDEX_DONE_CONTENTS
    }
}
