package test

var ()

const ()

func (h Heap) less(a, b int64) bool {
	i, j := h.I[a], h.I[b]
	return h.W[i] < h.W[j]
}

type Heap struct {
	I []int64

	W []int64
}
