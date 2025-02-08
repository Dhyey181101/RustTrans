package test 

import (
    "math"
)

var (
)

const (
)

func trianglesArea(t []float64) float64 {
	trianglesArea := 0.0
	for i := 0; i < len(t); i += 6 {
		trianglesArea += math.Abs((t[i]*(t[i+3]-t[i+5]) + t[i+2]*(t[i+5]-t[i+1]) + t[i+4]*(t[i+1]-t[i+3])) / 2)
	}
	return trianglesArea
}

