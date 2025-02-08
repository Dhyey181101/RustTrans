package test

import (
	"math"
)

var ()

const ()

func (v geo_r3_Vector) Distance(ov geo_r3_Vector) float64 { return v.Sub(ov).Norm() }

func (v geo_r3_Vector) Sub(ov geo_r3_Vector) geo_r3_Vector {
	return geo_r3_Vector{v.X - ov.X, v.Y - ov.Y, v.Z - ov.Z}
}

func (v geo_r3_Vector) Norm() float64 { return math.Sqrt(v.Dot(v)) }

func (v geo_r3_Vector) Dot(ov geo_r3_Vector) float64 {
	return float64(v.X*ov.X) + float64(v.Y*ov.Y) + float64(v.Z*ov.Z)
}

type geo_r3_Vector struct {
	X, Y, Z float64
}
