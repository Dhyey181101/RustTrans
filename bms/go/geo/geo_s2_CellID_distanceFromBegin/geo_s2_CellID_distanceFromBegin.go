package test

import "math/bits"

const (
	geo_s2_MaxLevel = 30
)

func (ci geo_s2_CellID) distanceFromBegin() int64 {
	return int64(ci >> uint64(2*(geo_s2_MaxLevel-ci.Level())+1))
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
