package test

import (
	"fmt"
	"math"
)

const (
	geo_s1_Radian geo_s1_Angle = 1
	geo_s1_Degree              = (math.Pi / 180) * geo_s1_Radian
)

func (c geo_s2_Cap) String() string {
	return fmt.Sprintf("[Center=%v, Radius=%f]", c.center.geo_r3_Vector, c.Radius().Degrees())
}

func (c geo_s2_Cap) Radius() geo_s1_Angle {
	return c.radius.Angle()
}

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

func (a geo_s1_Angle) Degrees() float64 { return float64(a / geo_s1_Degree) }

type geo_s2_Cap struct {
	center geo_s2_Point
	radius geo_s1_ChordAngle
}

type geo_s2_Point struct {
	geo_r3_Vector
}

type geo_r3_Vector struct {
}

type geo_s1_ChordAngle float64

type geo_s1_Angle float64
