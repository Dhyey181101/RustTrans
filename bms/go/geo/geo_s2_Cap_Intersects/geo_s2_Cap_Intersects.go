package test

import (
	"math"
)

// var ()

const (
	geo_s1_StraightChordAngle = geo_s1_ChordAngle(4)
	geo_s1_maxLength2         = 4.0
)

func (c geo_s2_Cap) Intersects(other geo_s2_Cap) bool {
	if c.IsEmpty() || other.IsEmpty() {
		return false
	}

	return c.radius.Add(other.radius) >= geo_s2_ChordAngleBetweenPoints(c.center, other.center)
}

func (c geo_s2_Cap) IsEmpty() bool {
	return c.radius < 0
}

func (c geo_s1_ChordAngle) Add(other geo_s1_ChordAngle) geo_s1_ChordAngle {
	// Note that this method (and Sub) is much more efficient than converting
	// the ChordAngle to an Angle and adding those and converting back. It
	// requires only one square root plus a few additions and multiplications.

	// Optimization for the common case where b is an error tolerance
	// parameter that happens to be set to zero.
	if other == 0 {
		return c
	}

	// Clamp the angle sum to at most 180 degrees.
	if c+other >= geo_s1_maxLength2 {
		return geo_s1_StraightChordAngle
	}

	// Let a and b be the (non-squared) chord lengths, and let c = a+b.
	// Let A, B, and C be the corresponding half-angles (a = 2*sin(A), etc).
	// Then the formula below can be derived from c = 2 * sin(A+B) and the
	// relationships   sin(A+B) = sin(A)*cos(B) + sin(B)*cos(A)
	//                 cos(X) = sqrt(1 - sin^2(X))
	x := float64(c * (1 - 0.25*other))
	y := float64(other * (1 - 0.25*c))
	return geo_s1_ChordAngle(math.Min(geo_s1_maxLength2, x+y+2*math.Sqrt(x*y)))
}

func geo_s2_ChordAngleBetweenPoints(x, y geo_s2_Point) geo_s1_ChordAngle {
	return geo_s1_ChordAngle(math.Min(4.0, x.Sub(y.geo_r3_Vector).Norm2()))
}

func (v geo_r3_Vector) Sub(ov geo_r3_Vector) geo_r3_Vector {
	return geo_r3_Vector{v.X - ov.X, v.Y - ov.Y, v.Z - ov.Z}
}

func (v geo_r3_Vector) Norm2() float64 { return v.Dot(v) }

func (v geo_r3_Vector) Dot(ov geo_r3_Vector) float64 {
	return float64(v.X*ov.X) + float64(v.Y*ov.Y) + float64(v.Z*ov.Z)
}

type geo_s2_Cap struct {
	center geo_s2_Point
	radius geo_s1_ChordAngle
}

type geo_s2_Point struct {
	geo_r3_Vector
}

type geo_r3_Vector struct {
	X, Y, Z float64
}

type geo_s1_ChordAngle float64
