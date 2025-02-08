package test

import (
	"math"
)

func (i geo_s1_Interval) ContainsInterval(oi geo_s1_Interval) bool {
	if i.IsInverted() {
		if oi.IsInverted() {
			return oi.Lo >= i.Lo && oi.Hi <= i.Hi
		}
		return (oi.Lo >= i.Lo || oi.Hi <= i.Hi) && !i.IsEmpty()
	}
	if oi.IsInverted() {
		return i.IsFull() || oi.IsEmpty()
	}
	return oi.Lo >= i.Lo && oi.Hi <= i.Hi
}

func (i geo_s1_Interval) IsInverted() bool { return i.Lo > i.Hi }

func (i geo_s1_Interval) IsEmpty() bool { return i.Lo == math.Pi && i.Hi == -math.Pi }

func (i geo_s1_Interval) IsFull() bool { return i.Lo == -math.Pi && i.Hi == math.Pi }

type geo_s1_Interval struct {
	Lo, Hi float64
}
