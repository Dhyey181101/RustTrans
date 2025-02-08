package test

import (
	"math"
)

const (
	geo_s1_StraightChordAngle = geo_s1_ChordAngle(4)
)

func (c geo_s2_Cap) InteriorContainsPoint(p geo_s2_Point) bool {
	return c.IsFull() || geo_s2_ChordAngleBetweenPoints(c.center, p) < c.radius
}

func (c geo_s2_Cap) IsFull() bool {
	return c.radius == geo_s1_StraightChordAngle
}

func geo_s2_ChordAngleBetweenPoints(x, y geo_s2_Point) geo_s1_ChordAngle {
	return geo_s1_ChordAngle(math.Min(4.0, x.Sub(y.geo_r3_Vector).Norm2()))
}

func (v geo_r3_Vector) Sub(ov geo_r3_Vector) geo_r3_Vector {
	return geo_r3_Vector{v.X - ov.X, v.Y - ov.Y, v.Z - ov.Z}
}

func (v geo_r3_Vector) Norm2() float64 { return v.Dot(v) }

func (v geo_r3_Vector) Dot(ov geo_r3_Vector) float64 {
	return float64(v.X*ov.X) + float64(v.Y*ov.Y) + float64(v.Z*ov.Z)
}

type geo_s2_Cap struct {
	center geo_s2_Point
	radius geo_s1_ChordAngle
}

type geo_s2_Point struct {
	geo_r3_Vector
}

type geo_r3_Vector struct {
	X, Y, Z float64
}

type geo_s1_ChordAngle float64
