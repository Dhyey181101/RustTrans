package test

import (
	"math"
)

func (i geo_s1_Interval) Intersection(oi geo_s1_Interval) geo_s1_Interval {
	if oi.IsEmpty() {
		return geo_s1_EmptyInterval()
	}
	if i.fastContains(oi.Lo) {
		if i.fastContains(oi.Hi) {
			// Either oi ⊂ i, or i and oi intersect twice. Neither are empty.
			// In the first case we want to return i (which is shorter than oi).
			// In the second case one of them is inverted, and the smallest interval
			// that covers the two disjoint pieces is the shorter of i and oi.
			// We thus want to pick the shorter of i and oi in both cases.
			if oi.Length() < i.Length() {
				return oi
			}
			return i
		}
		return geo_s1_Interval{oi.Lo, i.Hi}
	}
	if i.fastContains(oi.Hi) {
		return geo_s1_Interval{i.Lo, oi.Hi}
	}

	// Neither endpoint of oi is in i. Either i ⊂ oi, or i and oi are disjoint.
	if oi.fastContains(i.Lo) {
		return i
	}
	return geo_s1_EmptyInterval()
}

func (i geo_s1_Interval) IsEmpty() bool { return i.Lo == math.Pi && i.Hi == -math.Pi }

func geo_s1_EmptyInterval() geo_s1_Interval { return geo_s1_Interval{math.Pi, -math.Pi} }

func (i geo_s1_Interval) fastContains(p float64) bool {
	if i.IsInverted() {
		return (p >= i.Lo || p <= i.Hi) && !i.IsEmpty()
	}
	return p >= i.Lo && p <= i.Hi
}

func (i geo_s1_Interval) IsInverted() bool { return i.Lo > i.Hi }

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

type geo_s1_Interval struct {
	Lo, Hi float64
}
