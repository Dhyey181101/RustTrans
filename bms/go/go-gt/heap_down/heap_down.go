package test

var ()

const ()

func (h Heap) down(i int64) {
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

func (h Heap) less(a, b int64) bool {
	i, j := h.I[a], h.I[b]
	return h.W[i] < h.W[j]
}

func (h Heap) swap(a, b int64) {
	i, j := h.I[a], h.I[b]
	h.I[a], h.I[b] = h.I[b], h.I[a]
	h.A[i], h.A[j] = b, a
}

type Heap struct {
	N int64
	I []int64
	A []int64
	W []int64
}
