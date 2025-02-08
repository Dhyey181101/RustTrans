package test

import (
	"math"
)

// var ()

const (
	geo_s1_maxLength2 = 4.0
)

func (c geo_s1_ChordAngle) isValid() bool {
	return (c >= 0 && c <= geo_s1_maxLength2) || c.isSpecial()
}

func (c geo_s1_ChordAngle) isSpecial() bool {
	return c < 0 || c.IsInfinity()
}

func (c geo_s1_ChordAngle) IsInfinity() bool {
	return math.IsInf(float64(c), 1)
}

type geo_s1_ChordAngle float64
