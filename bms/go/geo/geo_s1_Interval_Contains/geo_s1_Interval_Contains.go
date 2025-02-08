package test

import (
	"math"
)

func (i geo_s1_Interval) Contains(p float64) bool {
	if p == -math.Pi {
		p = math.Pi
	}
	return i.fastContains(p)
}

func (i geo_s1_Interval) fastContains(p float64) bool {
	if i.IsInverted() {
		return (p >= i.Lo || p <= i.Hi) && !i.IsEmpty()
	}
	return p >= i.Lo && p <= i.Hi
}

func (i geo_s1_Interval) IsInverted() bool { return i.Lo > i.Hi }

func (i geo_s1_Interval) IsEmpty() bool { return i.Lo == math.Pi && i.Hi == -math.Pi }

type geo_s1_Interval struct {
	Lo, Hi float64
}
