package test

import (
	"math"
)

const (
	geo_s1_StraightChordAngle = geo_s1_ChordAngle(4)
)

func (c geo_s2_Cap) IsValid() bool {
	return c.center.geo_r3_Vector.IsUnit() && c.radius <= geo_s1_StraightChordAngle
}

func (v geo_r3_Vector) IsUnit() bool {
	const epsilon = 5e-14
	return math.Abs(v.Norm2()-1) <= epsilon
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

type geo_s2_typeTag uint32

type geo_s2_WedgeRel int

type geo_s2_CrossingType int

type geo_s2_axis int

type geo_s2_CellRelation int

type geo_s2_ShapeIndexIteratorPos int

type geo_s2_Direction int

type geo_s2_crossingTarget int

type geo_s2_Crossing int

type geo_s2_VertexModel int

type geo_r3_Axis int

type geo_s1_Angle float64
