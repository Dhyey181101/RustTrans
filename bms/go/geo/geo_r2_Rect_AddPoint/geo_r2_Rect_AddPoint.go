package test

var ()

const ()

func (r geo_r2_Rect) AddPoint(p geo_r2_Point) geo_r2_Rect {
	return geo_r2_Rect{r.X.AddPoint(p.X), r.Y.AddPoint(p.Y)}
}

func (i geo_r1_Interval) AddPoint(p float64) geo_r1_Interval {
	if i.IsEmpty() {
		return geo_r1_Interval{p, p}
	}
	if p < i.Lo {
		return geo_r1_Interval{p, i.Hi}
	}
	if p > i.Hi {
		return geo_r1_Interval{i.Lo, p}
	}
	return i
}

func (i geo_r1_Interval) IsEmpty() bool { return i.Lo > i.Hi }

type geo_r2_Rect struct {
	X, Y geo_r1_Interval
}

type geo_r1_Interval struct {
	Lo, Hi float64
}

type geo_r2_Point struct {
	X, Y float64
}
