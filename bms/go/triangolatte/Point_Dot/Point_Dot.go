package test

var ()

const ()

func (p Point) Dot(r Point) float64 {
	return p.X*r.X + p.Y*r.Y
}

type Point struct {
	X, Y float64
}
