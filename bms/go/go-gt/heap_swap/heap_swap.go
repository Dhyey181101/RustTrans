package test

var ()

const ()

func (h Heap) swap(a, b int64) {
	i, j := h.I[a], h.I[b]
	h.I[a], h.I[b] = h.I[b], h.I[a]
	h.A[i], h.A[j] = b, a
}

type Heap struct {
	I []int64
	A []int64
}
