package test

import (
	"math"
)

var ()

const ()

func newHeap(n int64) (h Heap) {
	h.N = n
	h.I = make([]int64, n)
	h.A = make([]int64, n)
	h.W = make([]int64, n)
	var i int64
	for i = 0; i < n; i++ {
		h.I[i] = i
		h.A[i] = i
		h.W[i] = math.MaxInt64
	}
	return h
}

type Heap struct {
	N int64
	I []int64
	A []int64
	W []int64
}
