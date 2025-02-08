package test

import (
	"fmt"
	"math"
	"math/rand"
)

var (
	Verbose bool
)

const ()

func QAP_SolveSA(a *Matrix, b *Matrix, p Vector, m int64) int64 {
	var i int64
	n := p.Len()
	w := make(Vector, n)
	w.Copy(p)
	cc := cost(a, b, p)
	c, dmin, dmax := initQAP(a, b, w, cc)
	var t0 float64 = float64(dmin + (dmax-dmin)/10.0)
	tf := float64(dmin)
	beta := (t0 - tf) / (float64(m) * t0 * tf)
	var fail int64 = 0
	tries := n * (n - 1) / 2
	tfound := t0
	var temp float64 = t0
	var r int64 = 0
	var s int64 = 1

	// SA iterations
	for i = 0; i < m; i++ {
		temp /= (beta*temp + 1)
		s++
		if s >= n {
			r++
			if r >= n-1 {
				r = 0
			}
			s = r + 1
		}
		d := delta(a, b, w, r, s)
		if (d < 0) || (rand.Float64() < math.Exp(-float64(d)/temp)) || (fail == tries) {
			c += d
			w.Swap(r, s)
			fail = 0
		} else {
			fail++
		}
		if fail == tries {
			beta = 0
			temp = tfound
		}

		// Best solution improved ?
		if c < cc {
			cc = c
			p.Copy(w)
			tfound = temp
			if Verbose {
				fmt.Printf("iteration %d: cost=%d\n", i, cc)
				p.Print()
			}
		}
	}
	return cc
}

func (v Vector) Len() int64 {
	return int64(len(v))
}

func (v Vector) Copy(w Vector) {
	for i := 0; i < len(v); i++ {
		v[i] = w[i]
	}
}

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

func (m Matrix) Get(i int64, j int64) int64 {
	return m.A[i*m.N+j]
}

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

func (v Vector) Print() {
	for i := 0; i < len(v); i++ {
		fmt.Printf("%d ", v[i])
	}
	fmt.Print("\n")
}

type Vector []int64

type Matrix struct {
	N int64
	A []int64
}
