package test

const (
	geo_s1_StraightChordAngle = geo_s1_ChordAngle(4)
	geo_s1_maxLength2         = 4.0
)

func geo_s2_CapFromCenterHeight(center geo_s2_Point, height float64) geo_s2_Cap {
	return geo_s2_CapFromCenterChordAngle(center, geo_s1_ChordAngleFromSquaredLength(2*height))
}

func geo_s1_ChordAngleFromSquaredLength(length2 float64) geo_s1_ChordAngle {
	if length2 > geo_s1_maxLength2 {
		return geo_s1_StraightChordAngle
	}
	return geo_s1_ChordAngle(length2)
}

func geo_s2_CapFromCenterChordAngle(center geo_s2_Point, radius geo_s1_ChordAngle) geo_s2_Cap {
	return geo_s2_Cap{
		center: center,
		radius: radius,
	}
}

type geo_s1_Angle float64

type geo_s1_ChordAngle float64

type geo_s2_Cap struct {
	center geo_s2_Point
	radius geo_s1_ChordAngle
}

type geo_s2_Point struct {
	geo_r3_Vector
}

type geo_r3_Vector struct {
}
