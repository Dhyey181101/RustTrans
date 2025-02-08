package test

var ()

const ()

func initTrace(n, inc int64, trace *Matrix) {
	var i, j int64
	for i = 0; i < n; i++ {
		for j = 0; j < n; j++ {
			trace.Set(i, j, inc)
		}
	}
}

func (m Matrix) Set(i int64, j int64, v int64) {
	m.A[i*m.N+j] = v
}

type Matrix struct {
	N int64
	A []int64
}
