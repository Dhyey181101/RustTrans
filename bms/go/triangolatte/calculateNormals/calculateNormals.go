package test

import (
	"math"
)

var ()

const ()

func calculateNormals(x, y float64) [2]Point {
	return [2]Point{
		Point{y, -x}.Normalize(),
		Point{-y, x}.Normalize(),
	}
}

func (p Point) Normalize() Point {
	return p.Scale(1)
}

func (p Point) Scale(f float64) Point {
	norm := float64(math.Sqrt(float64(p.X*p.X + p.Y*p.Y)))
	return Point{p.X / norm * f, p.Y / norm * f}
}

type Point struct {
	X, Y float64
}
