package test

var ()

const ()

func (G *Matrix) TransitiveClosure(N *Matrix) {
	var i, j, k int64
	for i = 0; i < G.N; i++ {
		for j = 0; j < G.N; j++ {
			for k = 0; k < G.N; k++ {
				if G.Get(i, k) > 0 && G.Get(k, j) > 0 {
					if G.Get(i, j) == 0 || G.Get(i, k)+G.Get(k, j) < G.Get(i, j) {
						G.Set(i, j, G.Get(i, k)+G.Get(k, j))
						if N != nil {
							N.Set(i, j, k+1)
						}
					}
				}
			}
		}
	}
}

func (m Matrix) Get(i int64, j int64) int64 {
	return m.A[i*m.N+j]
}

func (m Matrix) Set(i int64, j int64, v int64) {
	m.A[i*m.N+j] = v
}

type Matrix struct {
	N int64
	A []int64
}
