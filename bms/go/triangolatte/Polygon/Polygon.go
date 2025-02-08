package test

import (
	"errors"
	"math"
)

var ()

const ()

func Polygon(points []Point) ([]float64, error) {
	n := len(points)

	if n < 3 {
		return nil, errors.New("cannot triangulate less than three points")
	}

	// Allocate memory for all needed elements and initialize them by hand.
	elements := make([]Element, n)
	elements[0].Prev, elements[0].Next = &elements[n-1], &elements[1]
	elements[0].Point = points[0]
	for i := 1; i < n-1; i++ {
		elements[i].Prev, elements[i].Next = &elements[i-1], &elements[i+1]
		elements[i].Point = points[i]
	}
	elements[n-1].Prev, elements[n-1].Next = &elements[n-2], &elements[0]
	elements[n-1].Point = points[n-1]

	ear := &elements[0]

	// Any triangulation of simple polygon has n-2 triangles. Triangle has 3
	// two-dimensional coordinates.
	i, t := 0, make([]float64, (n-2)*6)

	stop := ear
	var prev, next *Element

	for ear.Prev != ear.Next {
		prev = ear.Prev
		next = ear.Next

		if isEar(ear) {
			if polygonArea([]Point{prev.Point, ear.Point, next.Point}) > 0 {
				t[i+0], t[i+1] = prev.Point.X, prev.Point.Y
				t[i+2], t[i+3] = ear.Point.X, ear.Point.Y
				t[i+4], t[i+5] = next.Point.X, next.Point.Y
				i += 6
			}

			ear.Remove()
			ear = ear.Next
			stop = stop.Next
			continue
		}

		ear = next

		if ear == stop {
			return []float64{}, errors.New("oops")
		}
	}

	// Return array slice of size consisting only of the elements actually took by
	// the triangulation (sometimes the number of triangles is lower than n-2 and
	// zeroes are filling the rest of the array).
	return t[0:i], nil
}

func isEar(p *Element) bool {
	a, b, c := p.Prev.Point, p.Point, p.Next.Point
	if isReflex(a, b, c) {
		return false
	}

	r := p.Next.Next
	for r != p.Prev {
		inside := isInsideTriangle(a, b, c, r.Point)
		reflex := isReflex(r.Prev.Point, r.Point, r.Next.Point)
		if inside && reflex {
			return false
		}
		r = r.Next
	}
	return true
}

func isReflex(a, b, c Point) bool {
	return (b.X-a.X)*(c.Y-b.Y)-(c.X-b.X)*(b.Y-a.Y) < 0
}

func isInsideTriangle(a, b, c, p Point) bool {
	return (c.X-p.X)*(a.Y-p.Y)-(a.X-p.X)*(c.Y-p.Y) >= 0 &&
		(a.X-p.X)*(b.Y-p.Y)-(b.X-p.X)*(a.Y-p.Y) >= 0 &&
		(b.X-p.X)*(c.Y-p.Y)-(c.X-p.X)*(b.Y-p.Y) >= 0
}

func polygonArea(data []Point) float64 {
	area := 0.0
	for i, j := 0, len(data)-1; i < len(data); i++ {
		area += data[i].X*data[j].Y - data[i].Y*data[j].X
		j = i
	}
	return math.Abs(area / 2)
}

func (e *Element) Remove() {
	e.Next.Prev = e.Prev
	e.Prev.Next = e.Next
}

type Point struct {
	X, Y float64
}

type Element struct {
	Prev, Next *Element
	Point      Point
}
