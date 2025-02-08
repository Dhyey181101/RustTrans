package test

var ()

const ()

func (polygons byMaxX) Len() int {
	return len(polygons)
}

type byMaxX [][]Point

type Point struct {
	X, Y float64
}
