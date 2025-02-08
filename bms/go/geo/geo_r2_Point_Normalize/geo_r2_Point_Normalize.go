package test

import (
	"math"
)

func (p geo_r2_Point) Normalize() geo_r2_Point {
	if p.X == 0 && p.Y == 0 {
		return p
	}
	return p.Mul(1 / p.Norm())
}

func (p geo_r2_Point) Norm() float64 { return math.Hypot(p.X, p.Y) }

func (p geo_r2_Point) Mul(m float64) geo_r2_Point { return geo_r2_Point{m * p.X, m * p.Y} }

type geo_r2_Point struct {
	X, Y float64
}
