package test

import "math/big"

var ()

const (
	geo_r3_prec = big.MaxPrec
)

func (v geo_r3_PreciseVector) Dot(ov geo_r3_PreciseVector) *big.Float {
	return geo_r3_precAdd(geo_r3_precMul(v.X, ov.X), geo_r3_precAdd(geo_r3_precMul(v.Y, ov.Y), geo_r3_precMul(v.Z, ov.Z)))
}

func geo_r3_precMul(a, b *big.Float) *big.Float {
	return new(big.Float).SetPrec(geo_r3_prec).Mul(a, b)
}

func geo_r3_precAdd(a, b *big.Float) *big.Float {
	return new(big.Float).SetPrec(geo_r3_prec).Add(a, b)
}

type geo_r3_PreciseVector struct {
	X, Y, Z *big.Float
}
