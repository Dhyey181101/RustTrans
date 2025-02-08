package test

import (
	"math"
	"math/big"
)

func (v geo_r3_PreciseVector) Vector() geo_r3_Vector {
	// The accuracy flag is ignored on these conversions back to float64.
	x, _ := v.X.Float64()
	y, _ := v.Y.Float64()
	z, _ := v.Z.Float64()
	return geo_r3_Vector{x, y, z}.Normalize()
}

func (v geo_r3_Vector) Normalize() geo_r3_Vector {
	n2 := v.Norm2()
	if n2 == 0 {
		return geo_r3_Vector{0, 0, 0}
	}
	return v.Mul(1 / math.Sqrt(n2))
}

func (v geo_r3_Vector) Norm2() float64 { return v.Dot(v) }

func (v geo_r3_Vector) Dot(ov geo_r3_Vector) float64 {
	return float64(v.X*ov.X) + float64(v.Y*ov.Y) + float64(v.Z*ov.Z)
}

func (v geo_r3_Vector) Mul(m float64) geo_r3_Vector { return geo_r3_Vector{m * v.X, m * v.Y, m * v.Z} }

type geo_r3_PreciseVector struct {
	X, Y, Z *big.Float
}

type geo_r3_Vector struct {
	X, Y, Z float64
}
