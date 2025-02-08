package test

import (
	"math"
)

func (c geo_s1_ChordAngle) Tan() float64 {
	return c.Sin() / c.Cos()
}

func (c geo_s1_ChordAngle) Sin() float64 {
	return math.Sqrt(c.Sin2())
}

func (c geo_s1_ChordAngle) Sin2() float64 {
	// Let a be the (non-squared) chord length, and let A be the corresponding
	// half-angle (a = 2*sin(A)). The formula below can be derived from:
	//   sin(2*A) = 2 * sin(A) * cos(A)
	//   cos^2(A) = 1 - sin^2(A)
	// This is much faster than converting to an angle and computing its sine.
	return float64(c * (1 - 0.25*c))
}

func (c geo_s1_ChordAngle) Cos() float64 {
	// cos(2*A) = cos^2(A) - sin^2(A) = 1 - 2*sin^2(A)
	return float64(1 - 0.5*c)
}

type geo_s1_ChordAngle float64
