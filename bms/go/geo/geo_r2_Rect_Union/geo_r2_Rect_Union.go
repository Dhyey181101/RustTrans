package test

import (
	"math"
)

var ()

const ()

func (r geo_r2_Rect) Union(other geo_r2_Rect) geo_r2_Rect {
	return geo_r2_Rect{r.X.Union(other.X), r.Y.Union(other.Y)}
}

func (i geo_r1_Interval) Union(other geo_r1_Interval) geo_r1_Interval {
	if i.IsEmpty() {
		return other
	}
	if other.IsEmpty() {
		return i
	}
	return geo_r1_Interval{math.Min(i.Lo, other.Lo), math.Max(i.Hi, other.Hi)}
}

func (i geo_r1_Interval) IsEmpty() bool { return i.Lo > i.Hi }

type geo_r2_Rect struct {
	X, Y geo_r1_Interval
}

type geo_r1_Interval struct {
	Lo, Hi float64
}
