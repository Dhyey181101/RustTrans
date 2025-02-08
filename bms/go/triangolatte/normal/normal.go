package test

import (
	"math"
)

var ()

const ()

func normal(points []Point, width float64) []float64 {
	width /= 2.0
	triangles := make([]float64, 0, len(points)*12)
	for i := 0; i <= len(points)-2; i++ {
		dx := points[i+1].X - points[i].X
		dy := points[i+1].Y - points[i].Y
		n1 := Point{dy, -dx}.Scale(width)
		n2 := Point{-dy, dx}.Scale(width)

		v0, v1 := points[i+1].Add(n2).X, points[i+1].Add(n2).Y
		v2, v3 := points[i].Add(n2).X, points[i].Add(n2).Y
		v4, v5 := points[i].Add(n1).X, points[i].Add(n1).Y
		v6, v7 := points[i].Add(n1).X, points[i].Add(n1).Y
		v8, v9 := points[i+1].Add(n1).X, points[i+1].Add(n1).Y
		v10, v11 := points[i+1].Add(n2).X, points[i+1].Add(n2).Y

		triangles = append(triangles, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
	}

	return triangles
}

func (p Point) Scale(f float64) Point {
	norm := float64(math.Sqrt(float64(p.X*p.X + p.Y*p.Y)))
	return Point{p.X / norm * f, p.Y / norm * f}
}

func (p Point) Add(r Point) Point {
	return Point{p.X + r.X, p.Y + r.Y}
}

type Point struct {
	X, Y float64
}
