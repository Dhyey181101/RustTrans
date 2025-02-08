package test

import (
	"math"
)

var (
	geo_s1_epsilon = 1e-15
)

func (i geo_s1_Interval) ApproxEqual(other geo_s1_Interval) bool {
	// Full and empty intervals require special cases because the endpoints
	// are considered to be positioned arbitrarily.
	if i.IsEmpty() {
		return other.Length() <= 2*geo_s1_epsilon
	}
	if other.IsEmpty() {
		return i.Length() <= 2*geo_s1_epsilon
	}
	if i.IsFull() {
		return other.Length() >= 2*(math.Pi-geo_s1_epsilon)
	}
	if other.IsFull() {
		return i.Length() >= 2*(math.Pi-geo_s1_epsilon)
	}

	// The purpose of the last test below is to verify that moving the endpoints
	// does not invert the interval, e.g. [-1e20, 1e20] vs. [1e20, -1e20].
	return (math.Abs(math.Remainder(other.Lo-i.Lo, 2*math.Pi)) <= geo_s1_epsilon &&
		math.Abs(math.Remainder(other.Hi-i.Hi, 2*math.Pi)) <= geo_s1_epsilon &&
		math.Abs(i.Length()-other.Length()) <= 2*geo_s1_epsilon)

}

func (i geo_s1_Interval) IsEmpty() bool { return i.Lo == math.Pi && i.Hi == -math.Pi }

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

func (i geo_s1_Interval) IsFull() bool { return i.Lo == -math.Pi && i.Hi == math.Pi }

type geo_s1_Interval struct {
	Lo, Hi float64
}
