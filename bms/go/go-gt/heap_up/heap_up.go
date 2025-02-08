package test

var ()

const ()

func (h Heap) up(j int64) {
	for {
		i := (j - 1) / 2
		if i == j || h.less(i, j) {
			break
		}
		h.swap(i, j)
		j = i
	}
}

func (h Heap) swap(a, b int64) {
	i, j := h.I[a], h.I[b]
	h.I[a], h.I[b] = h.I[b], h.I[a]
	h.A[i], h.A[j] = b, a
}

func (h Heap) less(a, b int64) bool {
	i, j := h.I[a], h.I[b]
	return h.W[i] < h.W[j]
}

type Heap struct {
	I []int64
	A []int64
	W []int64
}
