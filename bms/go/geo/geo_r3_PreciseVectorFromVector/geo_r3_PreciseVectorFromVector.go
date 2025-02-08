package test

import "math/big"

var ()

const (
	geo_r3_prec = big.MaxPrec
)

func geo_r3_PreciseVectorFromVector(v geo_r3_Vector) geo_r3_PreciseVector {
	return geo_r3_NewPreciseVector(v.X, v.Y, v.Z)
}

func geo_r3_NewPreciseVector(x, y, z float64) geo_r3_PreciseVector {
	return geo_r3_PreciseVector{
		X: geo_r3_precFloat(x),
		Y: geo_r3_precFloat(y),
		Z: geo_r3_precFloat(z),
	}
}

func geo_r3_precFloat(f float64) *big.Float {
	return new(big.Float).SetPrec(geo_r3_prec).SetFloat64(f)
}

type geo_r3_Vector struct {
	X, Y, Z float64
}

type geo_r3_PreciseVector struct {
	X, Y, Z *big.Float
}
