package test

import (
	"math"
)

func (r geo_r2_Rect) Intersection(other geo_r2_Rect) geo_r2_Rect {
	xx := r.X.Intersection(other.X)
	yy := r.Y.Intersection(other.Y)
	if xx.IsEmpty() || yy.IsEmpty() {
		return geo_r2_EmptyRect()
	}

	return geo_r2_Rect{xx, yy}
}

func (i geo_r1_Interval) Intersection(j geo_r1_Interval) geo_r1_Interval {
	// Empty intervals do not need to be special-cased.
	return geo_r1_Interval{
		Lo: math.Max(i.Lo, j.Lo),
		Hi: math.Min(i.Hi, j.Hi),
	}
}

func (i geo_r1_Interval) IsEmpty() bool { return i.Lo > i.Hi }

func geo_r2_EmptyRect() geo_r2_Rect {
	return geo_r2_Rect{geo_r1_EmptyInterval(), geo_r1_EmptyInterval()}
}

func geo_r1_EmptyInterval() geo_r1_Interval { return geo_r1_Interval{1, 0} }

type geo_r2_Rect struct {
	X, Y geo_r1_Interval
}

type geo_r1_Interval struct {
	Lo, Hi float64
}
