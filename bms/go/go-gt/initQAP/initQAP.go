package test

import "math/rand"

var ()

const ()

type Vector []int64

func initQAP(a *Matrix, b *Matrix, w Vector, c int64) (int64, int64, int64) {
	var (
		dmin, dmax int64
	)
	n := w.Len()
	for i := 0; i < 10000; i++ {
		r := rand.Int63n(n)
		s := rand.Int63n(n - 1)
		if s >= r {
			s = s + 1
		}
		d := delta(a, b, w, r, s)
		c += d
		dmin = min(dmin, d)
		dmax = max(dmax, d)
		w.Swap(r, s)
	}
	return c, dmin, dmax
}

func (v Vector) Len() int64 {
	return int64(len(v))
}

func delta(a *Matrix, b *Matrix, p Vector, r int64, s int64) (d int64) {
	var i int64
	d = int64((a.Get(r, r)-a.Get(s, s))*(b.Get(p[s], p[s])-b.Get(p[r], p[r])) +
		(a.Get(r, s)-a.Get(s, r))*(b.Get(p[s], p[r])-b.Get(p[r], p[s])))
	for i = 0; i < p.Len(); i++ {
		if i != r && i != s {
			d += (a.Get(i, r)-a.Get(i, s))*(b.Get(p[i], p[s])-b.Get(p[i], p[r])) +
				(a.Get(r, i)-a.Get(s, i))*(b.Get(p[s], p[i])-b.Get(p[r], p[i]))
		}
	}
	return d
}

func (m Matrix) Get(i int64, j int64) int64 {
	return m.A[i*m.N+j]
}

func min(a, b int64) int64 {
	if a < b {
		return a
	}
	return b
}

func max(a, b int64) int64 {
	if a > b {
		return a
	}
	return b
}

func (p Vector) Swap(i int64, j int64) {
	x := p[i]
	p[i] = p[j]
	p[j] = x
}

type Matrix struct {
	N int64
	A []int64
}
