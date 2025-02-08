package test

import (
	"math"
)

func (c geo_s2_Cap) Centroid() geo_s2_Point {
	// From symmetry, the centroid of the cap must be somewhere on the line
	// from the origin to the center of the cap on the surface of the sphere.
	// When a sphere is divided into slices of constant thickness by a set of
	// parallel planes, all slices have the same surface area. This implies
	// that the radial component of the centroid is simply the midpoint of the
	// range of radial distances spanned by the cap. That is easily computed
	// from the cap height.
	if c.IsEmpty() {
		return geo_s2_Point{}
	}
	r := 1 - 0.5*c.Height()
	return geo_s2_Point{c.center.Mul(r * c.Area())}
}

func (c geo_s2_Cap) IsEmpty() bool {
	return c.radius < 0
}

func (c geo_s2_Cap) Height() float64 {
	return float64(0.5 * c.radius)
}

func (c geo_s2_Cap) Area() float64 {
	return 2.0 * math.Pi * math.Max(0, c.Height())
}

func (v geo_r3_Vector) Mul(m float64) geo_r3_Vector { return geo_r3_Vector{m * v.X, m * v.Y, m * v.Z} }

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
