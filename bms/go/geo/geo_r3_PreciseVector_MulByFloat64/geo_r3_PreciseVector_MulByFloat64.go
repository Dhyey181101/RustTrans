package test

import "math/big"

var ()

const (
	geo_r3_prec = big.MaxPrec
)

func (v geo_r3_PreciseVector) MulByFloat64(f float64) geo_r3_PreciseVector {
	return v.Mul(geo_r3_precFloat(f))
}

func geo_r3_precFloat(f float64) *big.Float {
	return new(big.Float).SetPrec(geo_r3_prec).SetFloat64(f)
}

func (v geo_r3_PreciseVector) Mul(f *big.Float) geo_r3_PreciseVector {
	return geo_r3_PreciseVector{
		X: geo_r3_precMul(v.X, f),
		Y: geo_r3_precMul(v.Y, f),
		Z: geo_r3_precMul(v.Z, f),
	}
}

func geo_r3_precMul(a, b *big.Float) *big.Float {
	return new(big.Float).SetPrec(geo_r3_prec).Mul(a, b)
}

type geo_r3_PreciseVector struct {
	X, Y, Z *big.Float
}
