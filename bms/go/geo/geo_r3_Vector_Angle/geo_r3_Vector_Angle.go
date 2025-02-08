package test

import (
	"math"
)

var ()

const (
	geo_s1_Radian geo_s1_Angle = 1
)

func (v geo_r3_Vector) Angle(ov geo_r3_Vector) geo_s1_Angle {
	return geo_s1_Angle(math.Atan2(v.Cross(ov).Norm(), v.Dot(ov))) * geo_s1_Radian
}

func (v geo_r3_Vector) Cross(ov geo_r3_Vector) geo_r3_Vector {
	return geo_r3_Vector{
		float64(v.Y*ov.Z) - float64(v.Z*ov.Y),
		float64(v.Z*ov.X) - float64(v.X*ov.Z),
		float64(v.X*ov.Y) - float64(v.Y*ov.X),
	}
}

func (v geo_r3_Vector) Norm() float64 { return math.Sqrt(v.Dot(v)) }

func (v geo_r3_Vector) Dot(ov geo_r3_Vector) float64 {
	return float64(v.X*ov.X) + float64(v.Y*ov.Y) + float64(v.Z*ov.Z)
}

type geo_r3_Vector struct {
	X, Y, Z float64
}

type geo_s1_Angle float64
