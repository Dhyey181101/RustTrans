package test

import (
	"math"
)

func (v geo_r3_Vector) IsUnit() bool {
	const epsilon = 5e-14
	return math.Abs(v.Norm2()-1) <= epsilon
}

func (v geo_r3_Vector) Norm2() float64 { return v.Dot(v) }

func (v geo_r3_Vector) Dot(ov geo_r3_Vector) float64 {
	return float64(v.X*ov.X) + float64(v.Y*ov.Y) + float64(v.Z*ov.Z)
}

type geo_r3_Vector struct {
	X, Y, Z float64
}
