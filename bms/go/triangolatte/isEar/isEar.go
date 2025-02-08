package test

var ()

const ()

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

type Element struct {
	Prev, Next *Element
	Point      Point
}

type Point struct {
	X, Y float64
}
