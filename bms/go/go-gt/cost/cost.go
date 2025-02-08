package test

var ()

const ()

func cost(a *Matrix, b *Matrix, p Vector) (c int64) {
	var i, j int64
	c = 0
	for i = 0; i < p.Len(); i++ {
		for j = 0; j < p.Len(); j++ {
			c += a.Get(i, j) * b.Get(p[i], p[j])
		}
	}
	return c
}

func (v Vector) Len() int64 {
	return int64(len(v))
}

func (m Matrix) Get(i int64, j int64) int64 {
	return m.A[i*m.N+j]
}

type Vector []int64

type Matrix struct {
	N int64
	A []int64
}
