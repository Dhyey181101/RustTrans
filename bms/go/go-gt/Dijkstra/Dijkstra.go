package test

import (
	"math"
)

var ()

const ()

func Dijkstra(G *Matrix, i int64) (p []int64) {
	p = make([]int64, G.N)
	h := newHeap(G.N)
	h.W[i] = 0
	h.swap(i, 0)
	for h.N > 0 {
		i = h.pop()
		if h.W[i] == math.MaxInt64 {
			return p
		}
		h.update(p, i, G)
	}
	return p
}

func newHeap(n int64) (h heap) {
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

func (h heap) swap(a, b int64) {
	i, j := h.I[a], h.I[b]
	h.I[a], h.I[b] = h.I[b], h.I[a]
	h.A[i], h.A[j] = b, a
}

func (h *heap) pop() (i int64) {
	i = h.I[0]
	h.N--
	h.swap(0, h.N)
	h.down(0)
	return i
}

func (h heap) down(i int64) {
	for {
		left := 2*i + 1
		if left >= h.N {
			break
		}
		j := left
		if right := left + 1; right < h.N && !h.less(left, right) {
			j = right
		}
		if h.less(i, j) {
			break
		}
		h.swap(i, j)
		i = j
	}
}

func (h heap) less(a, b int64) bool {
	i, j := h.I[a], h.I[b]
	return h.W[i] < h.W[j]
}

func (h heap) update(p []int64, i int64, G *Matrix) {
	var j int64
	for j = 0; j < G.N; j++ {
		if G.Get(i, j) > 0 {
			if h.W[i]+G.Get(i, j) < h.W[j] {
				p[j] = i + 1
				h.W[j] = h.W[i] + G.Get(i, j)
				h.up(h.A[j])
			}
		}
	}
}

func (m Matrix) Get(i int64, j int64) int64 {
	return m.A[i*m.N+j]
}

func (h heap) up(j int64) {
	for {
		i := (j - 1) / 2
		if i == j || h.less(i, j) {
			break
		}
		h.swap(i, j)
		j = i
	}
}

type Matrix struct {
	N int64
	A []int64
}

type heap struct {
	N int64
	I []int64
	A []int64
	W []int64
}
