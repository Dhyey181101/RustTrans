package test

import (
	"math"
)

var ()

const (
	originShift = 2.0 * math.Pi * 6378137 / 2.0
	math_Pi     = 3.14159265358979323846264338327950288419716939937510582097494459
)

func degreesToMeters(point Point) Point {
	return Point{
		point.X * originShift / 180.0,
		math.Log(math.Tan((90.0+point.Y)*math_Pi/360.0)) / (math_Pi / 180.0) * originShift / 180.0,
	}
}

type Point struct {
	X, Y float64
}
