package test 

import (
    "math"
)

var (
)

const (
)

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

