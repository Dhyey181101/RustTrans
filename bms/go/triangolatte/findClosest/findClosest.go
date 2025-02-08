package test

import "container/list"

var ()

const ()

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

func isInsideTriangle(a, b, c, p Point) bool {
	return (c.X-p.X)*(a.Y-p.Y)-(a.X-p.X)*(c.Y-p.Y) >= 0 &&
		(a.X-p.X)*(b.Y-p.Y)-(b.X-p.X)*(a.Y-p.Y) >= 0 &&
		(b.X-p.X)*(c.Y-p.Y)-(c.X-p.X)*(b.Y-p.Y) >= 0
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
