package test

var ()

const ()

type Vector []int64

func updateTrace(n int64, p, best_p Vector, inc *int64, r int64, trace *Matrix) {
	var i int64
	for i = 0; i < n && p[i] == best_p[i]; i++ { // skip
	}
	if i == n {
		(*inc)++
		initTrace(n, *inc, trace)
	} else {
		for i = 0; i < n; i++ {
			trace.Set(i, p[i], trace.Get(i, p[i])+*inc)
			trace.Set(i, best_p[i], trace.Get(i, best_p[i])+r)
		}
	}
}

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

func (m Matrix) Get(i int64, j int64) int64 {
	return m.A[i*m.N+j]
}

type Matrix struct {
	N int64
	A []int64
}
