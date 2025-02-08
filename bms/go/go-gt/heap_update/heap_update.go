package test

var ()

const ()

func (h Heap) update(p []int64, i int64, G *Matrix) {
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
	N int64
	I []int64
	A []int64
	W []int64
}

type Matrix struct {
	N int64
	A []int64
}
