package test

import (
	"math"
)

var ()

const ()

func (e *Env) update() {
	var i int64
	var d int64 = math.MaxInt64
	for i = 0; i < e.N; i++ {
		if !e.T[i] {
			d = min(d, e.Slack[i])
		}
	}
	for i = 0; i < e.N; i++ {
		if e.S[i] {
			e.Lx[i] -= d
		}
	}
	for i = 0; i < e.N; i++ {
		if e.T[i] {
			e.Ly[i] += d
		}
	}
	for i = 0; i < e.N; i++ {
		if !e.T[i] {
			e.Slack[i] -= d
		}
	}
}

func min(a, b int64) int64 {
	if a < b {
		return a
	}
	return b
}

type Env struct {
	N int64

	T, S  []bool
	Slack []int64

	Lx, Ly []int64
}
