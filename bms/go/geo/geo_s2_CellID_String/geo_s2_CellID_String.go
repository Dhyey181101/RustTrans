package test

import (
	"bytes"
	"math/bits"
	"strconv"
)

const (
	geo_s2_NumFaces = 6
	geo_s2_PosBits  = 2*geo_s2_MaxLevel + 1
	geo_s2_MaxLevel = 30
)

func (ci geo_s2_CellID) String() string {
	if !ci.IsValid() {
		return "Invalid: " + strconv.FormatInt(int64(ci), 16)
	}
	var b bytes.Buffer
	b.WriteByte("012345"[ci.Face()]) // values > 5 will have been picked off by !IsValid above
	b.WriteByte('/')
	for level := 1; level <= ci.Level(); level++ {
		b.WriteByte("0123"[ci.ChildPosition(level)])
	}
	return b.String()
}

func (ci geo_s2_CellID) IsValid() bool {
	return ci.Face() < geo_s2_NumFaces && (ci.lsb()&0x1555555555555555 != 0)
}

func (ci geo_s2_CellID) Face() int { return int(uint64(ci) >> geo_s2_PosBits) }

func (ci geo_s2_CellID) lsb() uint64 { return uint64(ci) & -uint64(ci) }

func (ci geo_s2_CellID) ChildPosition(level int) int {
	return int(uint64(ci)>>uint64(2*(geo_s2_MaxLevel-level)+1)) & 3
}

func (ci geo_s2_CellID) Level() int {
	return geo_s2_MaxLevel - geo_s2_findLSBSetNonZero64(uint64(ci))>>1
}

func geo_s2_findLSBSetNonZero64(x uint64) int {
	if x == 0 {
		return 0
	}
	return bits.TrailingZeros64(x)
}

type geo_s2_CellID uint64
