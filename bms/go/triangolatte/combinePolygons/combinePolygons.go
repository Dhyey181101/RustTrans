package test

import (
	"container/list"
	"errors"
)

var ()

const ()

func combinePolygons(outer, inner []Point) ([]Point, error) {
	xMax := 0.0
	mIndex := 0
	for i := 0; i < len(inner); i++ {
		if inner[i].X > xMax {
			xMax = inner[i].X
			mIndex = i
		}
	}

	m := inner[mIndex]

	var pIndex int
	visibleIndex := -1

	k, k1, k2, err := findK(m, outer)

	if err != nil {
		return nil, err
	}

	// If `K` is vertex of the outer polygon, `M` and `K` are mutually visible.
	for i := 0; i < len(outer); i++ {
		if outer[i] == k {
			visibleIndex = i
		}
	}

	// Otherwise, `K` is an interior point of the edge `[V_k_1, V_k_2]`. Find `P`
	// which is endpoint with greater x-value.
	if outer[k1].X > outer[k2].X {
		pIndex = k1
	} else {
		pIndex = k2
	}

	// Check with all vertices of the outer polygon to be outside of the
	// triangle `[M, K, P]`. If it is true, `M` and `P` are mutually visible.
	allOutside := areAllOutside(m, k, pIndex, outer)

	if visibleIndex < 0 && allOutside {
		visibleIndex = pIndex
	}

	// Otherwise at least one reflex vertex lies in `[M, K, P]`. Search for the
	// array of reflex vertices `R` that minimizes the angle between `(1, 0)` and
	// line segment `[M, R]`. If there is exactly one vertex in `R` then they are
	// mutually visible. If there are multiple such vertices, pick the one closest
	// to `M`.
	if visibleIndex < 0 {
		visibleIndex = findClosest(m, k, pIndex, outer)
	}

	if visibleIndex < 0 {
		return nil, errors.New("could not find visible vertex")
	}

	result := make([]Point, 0, len(outer)+len(inner)+2)
	result = append(result, outer[:visibleIndex+1]...)
	for i := 0; i < len(inner); i++ {
		result = append(result, inner[cyclic(mIndex+i, len(inner))])
	}
	result = append(result, inner[mIndex], outer[visibleIndex])
	result = append(result, outer[visibleIndex+1:]...)

	return result, nil
}

func findK(m Point, outer []Point) (k Point, k1, k2 int, err error) {
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

func areAllOutside(m, k Point, pIndex int, outer []Point) bool {
	allOutside := true
	for i := range outer {
		// We have to skip M, K and P vertices. Since M is from the inner
		// polygon and K was proved to not match any vertex, the only one to
		// check is pIndex
		if i == pIndex {
			continue
		}

		if isInsideTriangle(m, k, outer[pIndex], outer[i]) {
			allOutside = false
		}
	}
	return allOutside
}

func isInsideTriangle(a, b, c, p Point) bool {
	return (c.X-p.X)*(a.Y-p.Y)-(a.X-p.X)*(c.Y-p.Y) >= 0 &&
		(a.X-p.X)*(b.Y-p.Y)-(b.X-p.X)*(a.Y-p.Y) >= 0 &&
		(b.X-p.X)*(c.Y-p.Y)-(c.X-p.X)*(b.Y-p.Y) >= 0
}

func findClosest(m, k Point, pIndex int, outer []Point) int {
	reflex := list.New()
	n := len(outer)
	for i := 0; i < n; i++ {
		notInside := !isInsideTriangle(m, k, outer[pIndex], outer[i])
		prev, next := cyclic(i-1, n), cyclic(i+1, n)
		notReflex := !isReflex(outer[prev], outer[i], outer[next])
		if notInside || notReflex {
			continue
		}
		reflex.PushBack(i)
	}
	var closest int
	var maxDist float64

	for r := reflex.Front(); r != nil; r = r.Next() {
		i := r.Value.(int)
		dist := outer[i].Distance2(outer[closest])
		if dist > maxDist {
			closest = i
			maxDist = dist
		}
	}
	return closest
}

func cyclic(i, n int) int {
	return (i%n + n) % n
}

func isReflex(a, b, c Point) bool {
	return (b.X-a.X)*(c.Y-b.Y)-(c.X-b.X)*(b.Y-a.Y) < 0
}

func (p Point) Distance2(r Point) float64 {
	return (p.X-r.X)*(p.X-r.X) + (p.Y-r.Y)*(p.Y-r.Y)
}

type Point struct {
	X, Y float64
}
