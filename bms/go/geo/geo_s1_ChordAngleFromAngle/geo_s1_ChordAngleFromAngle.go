package test

import (
	"math"
)

var ()

const (
	geo_s1_NegativeChordAngle = geo_s1_ChordAngle(-1)
)

func geo_s1_ChordAngleFromAngle(a geo_s1_Angle) geo_s1_ChordAngle {
	if a < 0 {
		return geo_s1_NegativeChordAngle
	}
	if a.isInf() {
		return geo_s1_InfChordAngle()
	}
	l := 2 * math.Sin(0.5*math.Min(math.Pi, a.Radians()))
	return geo_s1_ChordAngle(l * l)
}

func (a geo_s1_Angle) isInf() bool {
	return math.IsInf(float64(a), 0)
}

func geo_s1_InfChordAngle() geo_s1_ChordAngle {
	return geo_s1_ChordAngle(math.Inf(1))
}

func (a geo_s1_Angle) Radians() float64 { return float64(a) }

type geo_s1_Angle float64

type geo_s1_ChordAngle float64
