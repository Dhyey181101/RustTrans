package test

import "math/big"

var (
	geo_r3_precise1 = geo_r3_precInt(1)
)

const (
	geo_r3_prec = big.MaxPrec
)

func (v geo_r3_PreciseVector) IsUnit() bool {
	return v.Norm2().Cmp(geo_r3_precise1) == 0
}

func (v geo_r3_PreciseVector) Norm2() *big.Float { return v.Dot(v) }

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

func geo_r3_precInt(i int64) *big.Float {
	return new(big.Float).SetPrec(geo_r3_prec).SetInt64(i)
}
