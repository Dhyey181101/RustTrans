package test

import (
	"math"
)

const (
	geo_s1_Radian geo_s1_Angle = 1
)

func (c geo_s1_ChordAngle) Angle() geo_s1_Angle {
	if c < 0 {
		return -1 * geo_s1_Radian
	}
	if c.IsInfinity() {
		return geo_s1_InfAngle()
	}
	return geo_s1_Angle(2 * math.Asin(0.5*math.Sqrt(float64(c))))
}

func (c geo_s1_ChordAngle) IsInfinity() bool {
	return math.IsInf(float64(c), 1)
}

func geo_s1_InfAngle() geo_s1_Angle {
	return geo_s1_Angle(math.Inf(1))
}

type geo_s1_ChordAngle float64

type geo_s1_Angle float64
