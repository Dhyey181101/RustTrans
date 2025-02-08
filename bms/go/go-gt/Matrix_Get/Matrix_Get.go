package test

var ()

const ()

func (m Matrix) Get(i int64, j int64) int64 {
	return m.A[i*m.N+j]
}

type Matrix struct {
	N int64
	A []int64
}
