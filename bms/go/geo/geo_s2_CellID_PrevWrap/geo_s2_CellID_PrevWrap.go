package test

const (
	geo_s2_NumFaces   = 6
	geo_s2_MaxLevel   = 30
	geo_s2_PosBits    = 2*geo_s2_MaxLevel + 1
	geo_s2_wrapOffset = uint64(geo_s2_NumFaces) << geo_s2_PosBits
)

func (ci geo_s2_CellID) PrevWrap() geo_s2_CellID {
	p := ci.Prev()
	if uint64(p) < geo_s2_wrapOffset {
		return p
	}
	return geo_s2_CellID(uint64(p) + geo_s2_wrapOffset)
}

func (ci geo_s2_CellID) Prev() geo_s2_CellID {
	return geo_s2_CellID(uint64(ci) - ci.lsb()<<1)
}

func (ci geo_s2_CellID) lsb() uint64 { return uint64(ci) & -uint64(ci) }

type geo_s2_CellID uint64
