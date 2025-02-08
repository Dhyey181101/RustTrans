package test

var ()

const ()

func (polygons byMaxX) Swap(i, j int) {
	polygons[i], polygons[j] = polygons[j], polygons[i]
}

type byMaxX [][]Point

type Point struct {
	X, Y float64
}
