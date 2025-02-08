package test

import "math/rand"

var ()

const ()

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

func (v Vector) Len() int64 {
	return int64(len(v))
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

type Vector []int64

type Matrix struct {
	N int64
	A []int64
}
