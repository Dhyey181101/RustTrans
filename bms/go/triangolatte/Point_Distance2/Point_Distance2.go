package test

var ()

const ()

func (p Point) Distance2(r Point) float64 {
	return (p.X-r.X)*(p.X-r.X) + (p.Y-r.Y)*(p.Y-r.Y)
}

type Point struct {
	X, Y float64
}
