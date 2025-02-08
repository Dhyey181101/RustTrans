package test

var ()

const ()

func (p Point) Sub(r Point) Point {
	return Point{p.X - r.X, p.Y - r.Y}
}

type Point struct {
	X, Y float64
}
