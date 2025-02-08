package test

import (
	"math"
)

const (
	geo_s1_Radian geo_s1_Angle = 1
	geo_s1_Degree              = (math.Pi / 180) * geo_s1_Radian
)

func (a geo_s1_Angle) E7() int32 { return geo_s1_round(a.Degrees() * 1e7) }

func (a geo_s1_Angle) Degrees() float64 { return float64(a / geo_s1_Degree) }

func geo_s1_round(val float64) int32 {
	if val < 0 {
		return int32(val - 0.5)
	}
	return int32(val + 0.5)
}

type geo_s1_Angle float64
