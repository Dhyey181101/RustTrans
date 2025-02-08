package test

import (
	"fmt"
)

func (r geo_r2_Rect) String() string { return fmt.Sprintf("[Lo%s, Hi%s]", r.Lo(), r.Hi()) }

func (r geo_r2_Rect) Lo() geo_r2_Point {
	return geo_r2_Point{r.X.Lo, r.Y.Lo}
}

func (r geo_r2_Rect) Hi() geo_r2_Point {
	return geo_r2_Point{r.X.Hi, r.Y.Hi}
}

type geo_r2_Rect struct {
	X, Y geo_r1_Interval
}

type geo_r1_Interval struct {
	Lo, Hi float64
}

type geo_r2_Point struct {
	X, Y float64
}
