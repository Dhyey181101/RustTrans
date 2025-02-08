package test

import "math/big"

var ()

const (
	geo_r3_prec = big.MaxPrec
)

func (v geo_r3_PreciseVector) Cross(ov geo_r3_PreciseVector) geo_r3_PreciseVector {
	return geo_r3_PreciseVector{
		X: geo_r3_precSub(geo_r3_precMul(v.Y, ov.Z), geo_r3_precMul(v.Z, ov.Y)),
		Y: geo_r3_precSub(geo_r3_precMul(v.Z, ov.X), geo_r3_precMul(v.X, ov.Z)),
		Z: geo_r3_precSub(geo_r3_precMul(v.X, ov.Y), geo_r3_precMul(v.Y, ov.X)),
	}
}

func geo_r3_precMul(a, b *big.Float) *big.Float {
	return new(big.Float).SetPrec(geo_r3_prec).Mul(a, b)
}

func geo_r3_precSub(a, b *big.Float) *big.Float {
	return new(big.Float).SetPrec(geo_r3_prec).Sub(a, b)
}

type geo_r3_PreciseVector struct {
	X, Y, Z *big.Float
}
