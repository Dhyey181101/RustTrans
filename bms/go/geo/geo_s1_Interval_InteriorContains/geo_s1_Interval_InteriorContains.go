package test

import (
	"math"
)

func (i geo_s1_Interval) InteriorContains(p float64) bool {
	if p == -math.Pi {
		p = math.Pi
	}
	if i.IsInverted() {
		return p > i.Lo || p < i.Hi
	}
	return (p > i.Lo && p < i.Hi) || i.IsFull()
}

func (i geo_s1_Interval) IsInverted() bool { return i.Lo > i.Hi }

func (i geo_s1_Interval) IsFull() bool { return i.Lo == -math.Pi && i.Hi == math.Pi }

type geo_s1_Interval struct {
	Lo, Hi float64
}
