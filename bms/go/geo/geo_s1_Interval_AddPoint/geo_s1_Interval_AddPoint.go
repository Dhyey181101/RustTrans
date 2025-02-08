package test

import (
	"math"
)

func (i geo_s1_Interval) AddPoint(p float64) geo_s1_Interval {
	if math.Abs(p) > math.Pi {
		return i
	}
	if p == -math.Pi {
		p = math.Pi
	}
	if i.fastContains(p) {
		return i
	}
	if i.IsEmpty() {
		return geo_s1_Interval{p, p}
	}
	if geo_s1_positiveDistance(p, i.Lo) < geo_s1_positiveDistance(i.Hi, p) {
		return geo_s1_Interval{p, i.Hi}
	}
	return geo_s1_Interval{i.Lo, p}
}

func (i geo_s1_Interval) fastContains(p float64) bool {
	if i.IsInverted() {
		return (p >= i.Lo || p <= i.Hi) && !i.IsEmpty()
	}
	return p >= i.Lo && p <= i.Hi
}

func (i geo_s1_Interval) IsInverted() bool { return i.Lo > i.Hi }

func (i geo_s1_Interval) IsEmpty() bool { return i.Lo == math.Pi && i.Hi == -math.Pi }

func geo_s1_positiveDistance(a, b float64) float64 {
	d := b - a
	if d >= 0 {
		return d
	}
	return (b + math.Pi) - (a - math.Pi)
}

type geo_s1_Interval struct {
	Lo, Hi float64
}
