package test

import (
	"errors"
	"math"
)

var ()

const (
	// Normal triangulates with no joint correction.
	Normal Joint = 0
	// Miter triangulates producing miter joints, i.e. extending the lines until
	// they meet at some point.
	Miter Joint = 1
)

func Line(joint Joint, points []Point, width float64) ([]float64, error) {
	switch joint {
	case Normal:
		return normal(points, width), nil
	case Miter:
		return miter(points, width), nil
	default:
		return nil, errors.New("Unrecognized joint type")
	}
}

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

func miter(points []Point, width float64) []float64 {
	width /= 2.0
	triangles := make([]float64, 0, len(points)*12)
	var dx, dy float64
	var miter1, miter2 Point
	var n1, n2 [2]Point

	dx = points[1].X - points[0].X
	dy = points[1].Y - points[0].Y

	n2 = calculateNormals(dx, dy)
	miter2 = n2[0].Scale(width)

	for i := 1; i < len(points)-1; i++ {
		// Shift calculated values.
		n1 = n2
		miter1 = miter2

		dx = points[i+1].X - points[i].X
		dy = points[i+1].Y - points[i].Y

		n2 = calculateNormals(dx, dy)

		// Find tangent vector to both lines in the middle point.
		tangent := (points[i+1].Sub(points[i])).Normalize().Add((points[i].Sub(points[i-1])).Normalize()).Normalize()

		// Miter vector is perpendicular to the tangent and crosses extensions of
		// normal-translated lines in miter join points.
		unitMiter := Point{-tangent.Y, tangent.X}

		// Length of the miter vector projected onto one of the normals.
		// Choice of normal is arbitrary, each of them would work.
		miterLength := float64(width) / unitMiter.Dot(n1[0])
		miter2 = unitMiter.Scale(miterLength)

		v0, v1 := points[i].Sub(miter2).X, points[i].Sub(miter2).Y
		v2, v3 := points[i-1].Sub(miter1).X, points[i-1].Sub(miter1).Y
		v4, v5 := points[i-1].Add(miter1).X, points[i-1].Add(miter1).Y
		v6, v7 := points[i-1].Add(miter1).X, points[i-1].Add(miter1).Y
		v8, v9 := points[i].Add(miter2).X, points[i].Add(miter2).Y
		v10, v11 := points[i].Sub(miter2).X, points[i].Sub(miter2).Y

		triangles = append(triangles, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
	}

	// Use last normal as another 'neutral element' for miter join.
	n := len(points)
	lastMiter := n2[0].Scale(width)

	v0, v1 := points[n-1].Sub(lastMiter).X, points[n-1].Sub(lastMiter).Y
	v2, v3 := points[n-2].Sub(miter1).X, points[n-2].Sub(miter1).Y
	v4, v5 := points[n-2].Add(miter1).X, points[n-2].Add(miter1).Y
	v6, v7 := points[n-2].Add(miter1).X, points[n-2].Add(miter1).Y
	v8, v9 := points[n-1].Add(lastMiter).X, points[n-1].Add(lastMiter).Y
	v10, v11 := points[n-1].Sub(lastMiter).X, points[n-1].Sub(lastMiter).Y

	triangles = append(triangles, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)

	return triangles
}

func calculateNormals(x, y float64) [2]Point {
	return [2]Point{
		Point{y, -x}.Normalize(),
		Point{-y, x}.Normalize(),
	}
}

func (p Point) Normalize() Point {
	return p.Scale(1)
}

func (p Point) Sub(r Point) Point {
	return Point{p.X - r.X, p.Y - r.Y}
}

func (p Point) Dot(r Point) float64 {
	return p.X*r.X + p.Y*r.Y
}

type Joint int

type Point struct {
	X, Y float64
}
