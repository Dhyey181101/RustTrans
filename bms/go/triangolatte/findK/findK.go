package test

import (
	"errors"
)

var ()

const ()

func findK(m Point, outer []Point) (k Point, k1 int, k2 int, err error) {
	for i, j := len(outer)-1, 0; j < len(outer); i, j = j, j+1 {
		// Skip edges that does not have their first point below `M` and the second
		// one above.
		if outer[i].Y > m.Y || outer[j].Y < m.Y {
			continue
		}

		// Calculate simplified intersection of ray (1, 0) and [V_i, V_j] segment.
		v1 := m.Sub(outer[i])
		v2 := outer[j].Sub(outer[i])

		t1 := v2.Cross(v1) / v2.Y
		t2 := v1.Y / v2.Y

		if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
			// If there is no current `k` candidate or this one is closer.
			if t1-m.X < k.X {
				k = Point{X: t1 + m.X, Y: m.Y}
				k1, k2 = i, j
				return
			}
		} else {
			err = errors.New("cannot calculate intersection, problematic data")
			return
		}
	}
	return
}

func (p Point) Sub(r Point) Point {
	return Point{p.X - r.X, p.Y - r.Y}
}

func (p Point) Cross(r Point) float64 {
	return p.X*r.Y - p.Y*r.X
}

type Point struct {
	X, Y float64
}
