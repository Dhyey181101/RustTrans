package test

var ()

const ()

func (r geo_r2_Rect) InteriorContains(other geo_r2_Rect) bool {
	return r.X.InteriorContainsInterval(other.X) && r.Y.InteriorContainsInterval(other.Y)
}

func (i geo_r1_Interval) InteriorContainsInterval(oi geo_r1_Interval) bool {
	if oi.IsEmpty() {
		return true
	}
	return i.Lo < oi.Lo && oi.Hi < i.Hi
}

func (i geo_r1_Interval) IsEmpty() bool { return i.Lo > i.Hi }

type geo_r2_Rect struct {
	X, Y geo_r1_Interval
}

type geo_r1_Interval struct {
	Lo, Hi float64
}
