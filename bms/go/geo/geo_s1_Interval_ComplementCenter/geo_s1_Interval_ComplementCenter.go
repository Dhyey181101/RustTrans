package test

import (
	"math"
)

func (i geo_s1_Interval) ComplementCenter() float64 {
	if i.Lo != i.Hi {
		return i.Complement().Center()
	}
	// Singleton. The interval just contains a single point.
	if i.Hi <= 0 {
		return i.Hi + math.Pi
	}
	return i.Hi - math.Pi
}

func (i geo_s1_Interval) Complement() geo_s1_Interval {
	if i.Lo == i.Hi {
		// Singleton. The interval just contains a single point.
		return geo_s1_FullInterval()
	}
	// Handles empty and full.
	return geo_s1_Interval{i.Hi, i.Lo}
}

func geo_s1_FullInterval() geo_s1_Interval { return geo_s1_Interval{-math.Pi, math.Pi} }

func (i geo_s1_Interval) Center() float64 {
	c := 0.5 * (i.Lo + i.Hi)
	if !i.IsInverted() {
		return c
	}
	if c <= 0 {
		return c + math.Pi
	}
	return c - math.Pi
}

func (i geo_s1_Interval) IsInverted() bool { return i.Lo > i.Hi }

type geo_s1_Interval struct {
	Lo, Hi float64
}
