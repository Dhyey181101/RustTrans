package test

var ()

const ()

func (p Point) Cross(r Point) float64 {
	return p.X*r.Y - p.Y*r.X
}

type Point struct {
	X, Y float64
}
