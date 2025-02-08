package test

var ()

const ()

func delta_part(a, b, dist *Matrix, p Vector, i, j, r, s int64) int64 {
	return (dist.Get(i, j) + (a.Get(r, i)-a.Get(r, j)+a.Get(s, j)-a.Get(s, i))*
		(b.Get(p[s], p[i])-b.Get(p[s], p[j])+b.Get(p[r], p[j])-b.Get(p[r], p[i])) +
		(a.Get(i, r)-a.Get(j, r)+a.Get(j, s)-a.Get(i, s))*
			(b.Get(p[i], p[s])-b.Get(p[j], p[s])+b.Get(p[j], p[r])-b.Get(p[i], p[r])))
}

func (m Matrix) Get(i int64, j int64) int64 {
	return m.A[i*m.N+j]
}

type Vector []int64

type Matrix struct {
	N int64
	A []int64
}
