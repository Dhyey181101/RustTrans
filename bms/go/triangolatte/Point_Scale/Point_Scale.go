package test 

import (
    "math"
)

var (
)

const (
)

func (p Point) Scale(f float64) Point {
	norm := float64(math.Sqrt(float64(p.X*p.X + p.Y*p.Y)))
	return Point{p.X / norm * f, p.Y / norm * f}
}

type Point struct {
	X, Y float64
}

