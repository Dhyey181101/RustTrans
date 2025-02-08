package test

import (
	"math"
)

const (
	geo_s1_maxLength2 = 4.0
)

func (c geo_s1_ChordAngle) Expanded(e float64) geo_s1_ChordAngle {
	// If the angle is special, don't change it. Otherwise clamp it to the valid range.
	if c.isSpecial() {
		return c
	}
	return geo_s1_ChordAngle(math.Max(0.0, math.Min(geo_s1_maxLength2, float64(c)+e)))
}

func (c geo_s1_ChordAngle) isSpecial() bool {
	return c < 0 || c.IsInfinity()
}

func (c geo_s1_ChordAngle) IsInfinity() bool {
	return math.IsInf(float64(c), 1)
}

type geo_s1_ChordAngle float64
