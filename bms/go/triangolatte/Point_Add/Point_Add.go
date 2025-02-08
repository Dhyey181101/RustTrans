package test

var ()

const ()

func (p Point) Add(r Point) Point {
	return Point{p.X + r.X, p.Y + r.Y}
}

type Point struct {
	X, Y float64
}
