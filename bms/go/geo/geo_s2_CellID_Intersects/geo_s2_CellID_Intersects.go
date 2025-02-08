package test

func (ci geo_s2_CellID) Intersects(oci geo_s2_CellID) bool {
	return uint64(oci.RangeMin()) <= uint64(ci.RangeMax()) && uint64(oci.RangeMax()) >= uint64(ci.RangeMin())
}

func (ci geo_s2_CellID) RangeMin() geo_s2_CellID { return geo_s2_CellID(uint64(ci) - (ci.lsb() - 1)) }

func (ci geo_s2_CellID) lsb() uint64 { return uint64(ci) & -uint64(ci) }

func (ci geo_s2_CellID) RangeMax() geo_s2_CellID { return geo_s2_CellID(uint64(ci) + (ci.lsb() - 1)) }

type geo_s2_CellID uint64
