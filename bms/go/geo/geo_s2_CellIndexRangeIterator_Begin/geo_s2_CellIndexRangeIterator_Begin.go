package test

const (
	geo_s2_cellIndexDoneContents = -1
)

func (c *geo_s2_CellIndexRangeIterator) Begin() {
	c.pos = 0
	for c.nonEmpty && c.IsEmpty() && !c.Done() {
		c.pos++
	}
}

func (c *geo_s2_CellIndexRangeIterator) Done() bool {
	return c.pos >= len(c.rangeNodes)-1
}

func (c *geo_s2_CellIndexRangeIterator) IsEmpty() bool {
	return c.rangeNodes[c.pos].contents == geo_s2_cellIndexDoneContents
}

type geo_s2_CellIndexRangeIterator struct {
	rangeNodes []geo_s2_rangeNode
	pos        int
	nonEmpty   bool
}

type geo_s2_rangeNode struct {
	// First leaf cell contained by this range.
	contents int32 // Contents of this node (an index within the cell tree).
}
