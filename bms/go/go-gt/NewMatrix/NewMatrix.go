package test

var ()

const ()

func NewMatrix(n int64) (m *Matrix) {
	m = new(Matrix)
	m.N = n
	m.A = make([]int64, n*n)
	return m
}

type Matrix struct {
	N int64
	A []int64
}
