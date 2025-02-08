package test 

import (
    "math"
)

var (
)

const (
)

func deviation(data []Point, holes [][]Point, t []float64) (
	actual,
	calculated,
	deviation float64,
) {
	calculated = trianglesArea(t)
	actual = polygonArea(data)
	for _, h := range holes {
		actual -= polygonArea(h)
	}

	deviation = math.Abs(calculated - actual)
	return
}

func trianglesArea(t []float64) float64 {
	trianglesArea := 0.0
	for i := 0; i < len(t); i += 6 {
		trianglesArea += math.Abs((t[i]*(t[i+3]-t[i+5]) + t[i+2]*(t[i+5]-t[i+1]) + t[i+4]*(t[i+1]-t[i+3])) / 2)
	}
	return trianglesArea
}

func polygonArea(data []Point) float64 {
	area := 0.0
	for i, j := 0, len(data)-1; i < len(data); i++ {
		area += data[i].X*data[j].Y - data[i].Y*data[j].X
		j = i
	}
	return math.Abs(area / 2)
}

type Point struct {
	X, Y float64
}

