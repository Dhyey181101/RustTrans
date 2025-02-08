package test

import (
	"math"
)

var ()

const (
	geo_s2_epsilon = 1e-15
	geo_r1_epsilon = 1e-15
)

func (r geo_r2_Rect) ApproxEqual(r2 geo_r2_Rect) bool {
	return r.X.ApproxEqual(r2.X) && r.Y.ApproxEqual(r2.Y)
}

func (i geo_r1_Interval) ApproxEqual(other geo_r1_Interval) bool {
	if i.IsEmpty() {
		return other.Length() <= 2*geo_r1_epsilon
	}
	if other.IsEmpty() {
		return i.Length() <= 2*geo_r1_epsilon
	}
	return math.Abs(other.Lo-i.Lo) <= geo_r1_epsilon &&
		math.Abs(other.Hi-i.Hi) <= geo_r1_epsilon
}

func (i geo_r1_Interval) IsEmpty() bool { return i.Lo > i.Hi }

func (i geo_r1_Interval) Length() float64 { return i.Hi - i.Lo }

type geo_r2_Rect struct {
	X, Y geo_r1_Interval
}

type geo_r1_Interval struct {
	Lo, Hi float64
}
