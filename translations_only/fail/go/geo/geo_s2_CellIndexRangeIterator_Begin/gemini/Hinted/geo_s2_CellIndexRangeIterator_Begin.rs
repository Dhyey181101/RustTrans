
const GEO_S2_CELL_INDEX_DONE_CONTENTS: i32 = -1;

pub struct GeoS2CellIndexRangeIterator {
    range_nodes: Vec<GeoS2RangeNode>,
    pos: usize,
    non_empty: bool,
}

impl GeoS2CellIndexRangeIterator {
    pub fn begin(&mut self) {
        self.pos = 0;
        while self.non_empty && self.is_empty() && !self.done() {
            self.pos += 1;
        }
    }

    pub fn done(&self) -> bool {
        self.pos >= self.range_nodes.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.range_nodes[self.pos].contents == GEO_S2_CELL_INDEX_DONE_CONTENTS
    }
}

pub struct GeoS2RangeNode {
    contents: i32,
}
