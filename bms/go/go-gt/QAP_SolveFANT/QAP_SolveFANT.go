package test

import (
	"fmt"
	"math"
	"math/rand"
)

var (
	Verbose bool
)

const Inf int64 = math.MaxInt64

func QAP_SolveFANT(a *Matrix, b *Matrix, p Vector, r, m int64) int64 {
	var inc, i, c int64
	n := p.Len()
	w := make(Vector, n)
	w.Copy(p)
	trace := NewMatrix(n)
	inc = 1
	initTrace(n, inc, trace)
	cc := Inf

	// FANT iterations
	for i = 0; i < m; i++ {
		// Build a new solution
		genTrace(w, trace)
		c = cost(a, b, w)
		// Improve solution with a local search
		localSearch(a, b, w, &c)
		// Best solution improved ?
		if c < cc {
			cc = c
			p.Copy(w)
			if Verbose {
				fmt.Printf("iteration %d: cost=%d\n", i, cc)
				p.Print()
			}
			inc = 1
			initTrace(n, inc, trace)
		} else {
			// Memory update
			updateTrace(n, w, p, &inc, r, trace)
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

func NewMatrix(n int64) (m *Matrix) {
	m = new(Matrix)
	m.N = n
	m.A = make([]int64, n*n)
	return m
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

func genTrace(p Vector, trace *Matrix) {
	var i, j, k, target, sum int64
	n := p.Len()
	nexti := make(Vector, n)
	nextj := make(Vector, n)
	sum_trace := make(Vector, n)

	Perm(nexti)
	Perm(nextj)
	for i = 0; i < n; i++ {
		for j = 0; j < n; j++ {
			sum_trace[i] += trace.Get(i, j)
		}
	}

	for i = 0; i < n; i++ {
		target = unif(0, sum_trace[nexti[i]]-1)
		j = i
		sum = trace.Get(nexti[i], nextj[j])
		for sum < target {
			j++
			sum += trace.Get(nexti[i], nextj[j])
		}
		p[nexti[i]] = nextj[j]
		for k = i; k < n; k++ {
			sum_trace[nexti[k]] -= trace.Get(nexti[k], nextj[j])
		}
		nextj.Swap(j, i)
	}
}

func Perm(p Vector) {
	n := int64(len(p))
	var i int64
	for i = 0; i < n; i++ {
		p[i] = int64(i)
	}
	for i = 0; i < n; i++ {
		p.Swap(i, i+rand.Int63n(n-i))
	}
}

func (p Vector) Swap(i int64, j int64) {
	x := p[i]
	p[i] = p[j]
	p[j] = x
}

func (m Matrix) Get(i int64, j int64) int64 {
	return m.A[i*m.N+j]
}

func unif(low, high int64) int64 {
	return low + int64(float64(high-low+1)*rand.Float64())
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
