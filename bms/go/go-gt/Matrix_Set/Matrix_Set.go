package test

var ()

const ()

func (m Matrix) Set(i int64, j int64, v int64) {
	m.A[i*m.N+j] = v
}

type Matrix struct {
	N int64
	A []int64
}
