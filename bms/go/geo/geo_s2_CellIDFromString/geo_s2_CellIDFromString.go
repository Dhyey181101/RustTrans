package test

const (
	geo_s2_MaxLevel = 30
	geo_s2_PosBits  = 2*geo_s2_MaxLevel + 1
)

func geo_s2_CellIDFromString(s string) geo_s2_CellID {
	level := len(s) - 2
	if level < 0 || level > geo_s2_MaxLevel {
		return geo_s2_CellID(0)
	}
	face := int(s[0] - '0')
	if face < 0 || face > 5 || s[1] != '/' {
		return geo_s2_CellID(0)
	}
	id := geo_s2_CellIDFromFace(face)
	for i := 2; i < len(s); i++ {
		childPos := s[i] - '0'
		if childPos < 0 || childPos > 3 {
			return geo_s2_CellID(0)
		}
		id = id.Children()[childPos]
	}
	return id
}

func geo_s2_CellIDFromFace(face int) geo_s2_CellID {
	return geo_s2_CellID((uint64(face) << geo_s2_PosBits) + geo_s2_lsbForLevel(0))
}

func geo_s2_lsbForLevel(level int) uint64 { return 1 << uint64(2*(geo_s2_MaxLevel-level)) }

func (ci geo_s2_CellID) Children() [4]geo_s2_CellID {
	var ch [4]geo_s2_CellID
	lsb := geo_s2_CellID(ci.lsb())
	ch[0] = ci - lsb + lsb>>2
	lsb >>= 1
	ch[1] = ch[0] + lsb
	ch[2] = ch[1] + lsb
	ch[3] = ch[2] + lsb
	return ch
}

func (ci geo_s2_CellID) lsb() uint64 { return uint64(ci) & -uint64(ci) }

type geo_s2_CellID uint64
