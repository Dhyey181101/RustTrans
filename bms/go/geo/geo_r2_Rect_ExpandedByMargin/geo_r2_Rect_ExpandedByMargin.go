package test

func (r geo_r2_Rect) ExpandedByMargin(margin float64) geo_r2_Rect {
	return r.Expanded(geo_r2_Point{margin, margin})
}

func (r geo_r2_Rect) Expanded(margin geo_r2_Point) geo_r2_Rect {
	xx := r.X.Expanded(margin.X)
	yy := r.Y.Expanded(margin.Y)
	if xx.IsEmpty() || yy.IsEmpty() {
		return geo_r2_EmptyRect()
	}
	return geo_r2_Rect{xx, yy}
}

func (i geo_r1_Interval) Expanded(margin float64) geo_r1_Interval {
	if i.IsEmpty() {
		return i
	}
	return geo_r1_Interval{i.Lo - margin, i.Hi + margin}
}

func (i geo_r1_Interval) IsEmpty() bool { return i.Lo > i.Hi }

func geo_r2_EmptyRect() geo_r2_Rect {
	return geo_r2_Rect{geo_r1_EmptyInterval(), geo_r1_EmptyInterval()}
}

func geo_r1_EmptyInterval() geo_r1_Interval { return geo_r1_Interval{1, 0} }

type geo_r2_Rect struct {
	X, Y geo_r1_Interval
}

type geo_r1_Interval struct {
	Lo, Hi float64
}

type geo_r2_Point struct {
	X, Y float64
}
