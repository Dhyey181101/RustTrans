package test

import (
	"math"
)

var (
	geo_s1_dblEpsilon = 2.220446049e-16
)

func (i geo_s1_Interval) Expanded(margin float64) geo_s1_Interval {
	if margin >= 0 {
		if i.IsEmpty() {
			return i
		}
		// Check whether this interval will be full after expansion, allowing
		// for a rounding error when computing each endpoint.
		if i.Length()+2*margin+2*geo_s1_dblEpsilon >= 2*math.Pi {
			return geo_s1_FullInterval()
		}
	} else {
		if i.IsFull() {
			return i
		}
		// Check whether this interval will be empty after expansion, allowing
		// for a rounding error when computing each endpoint.
		if i.Length()+2*margin-2*geo_s1_dblEpsilon <= 0 {
			return geo_s1_EmptyInterval()
		}
	}
	result := geo_s1_IntervalFromEndpoints(
		math.Remainder(i.Lo-margin, 2*math.Pi),
		math.Remainder(i.Hi+margin, 2*math.Pi),
	)
	if result.Lo <= -math.Pi {
		result.Lo = math.Pi
	}
	return result
}

func (i geo_s1_Interval) IsEmpty() bool { return i.Lo == math.Pi && i.Hi == -math.Pi }

func geo_s1_IntervalFromEndpoints(lo, hi float64) geo_s1_Interval {
	i := geo_s1_Interval{lo, hi}
	if lo == -math.Pi && hi != math.Pi {
		i.Lo = math.Pi
	}
	if hi == -math.Pi && lo != math.Pi {
		i.Hi = math.Pi
	}
	return i
}

func (i geo_s1_Interval) IsFull() bool { return i.Lo == -math.Pi && i.Hi == math.Pi }

func (i geo_s1_Interval) Length() float64 {
	l := i.Hi - i.Lo
	if l >= 0 {
		return l
	}
	l += 2 * math.Pi
	if l > 0 {
		return l
	}
	return -1
}

func geo_s1_FullInterval() geo_s1_Interval { return geo_s1_Interval{-math.Pi, math.Pi} }

func geo_s1_EmptyInterval() geo_s1_Interval { return geo_s1_Interval{math.Pi, -math.Pi} }

type geo_s1_Interval struct {
	Lo, Hi float64
}
