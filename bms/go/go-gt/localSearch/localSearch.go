package test

import "math/rand"

var ()

const ()

type Vector []int64

func localSearch(a *Matrix, b *Matrix, p Vector, cost *int64) {
	// set of moves, numbered from 0 to index
	var i, j, nMov int64
	n := p.Len()
	move := make(Vector, n*(n-1)/2)
	nMov = 0
	for i = 0; i < n-1; i++ {
		for j = i + 1; j < n; j++ {
			move[nMov] = n*i + j
			nMov++
		}
	}
	improved := true
	for k := 0; k < 2 && improved; k++ {
		improved = false
		for i = 0; i < nMov-1; i++ {
			move.Swap(i, unif(i+1, nMov-1))
		}
		for i = 0; i < nMov; i++ {
			r := move[i] / n
			s := move[i] % n
			d := delta(a, b, p, r, s)
			if d < 0 {
				*cost += d
				p.Swap(r, s)
				improved = true
			}
		}
	}
	return
}

func (v Vector) Len() int64 {
	return int64(len(v))
}

func unif(low, high int64) int64 {
	return low + int64(float64(high-low+1)*rand.Float64())
}

func (p Vector) Swap(i int64, j int64) {
	x := p[i]
	p[i] = p[j]
	p[j] = x
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

type Matrix struct {
	N int64
	A []int64
}
