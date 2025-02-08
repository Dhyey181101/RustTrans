package test

import (
	"math"
)

func (i geo_s1_Interval) InteriorIntersects(oi geo_s1_Interval) bool {
	if i.IsEmpty() || oi.IsEmpty() || i.Lo == i.Hi {
		return false
	}
	if i.IsInverted() {
		return oi.IsInverted() || oi.Lo < i.Hi || oi.Hi > i.Lo
	}
	if oi.IsInverted() {
		return oi.Lo < i.Hi || oi.Hi > i.Lo
	}
	return (oi.Lo < i.Hi && oi.Hi > i.Lo) || i.IsFull()
}

func (i geo_s1_Interval) IsEmpty() bool { return i.Lo == math.Pi && i.Hi == -math.Pi }

func (i geo_s1_Interval) IsInverted() bool { return i.Lo > i.Hi }

func (i geo_s1_Interval) IsFull() bool { return i.Lo == -math.Pi && i.Hi == math.Pi }

type geo_s1_Interval struct {
	Lo, Hi float64
}
