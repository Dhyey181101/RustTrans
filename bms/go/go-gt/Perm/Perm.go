package test

import "math/rand"

var ()

const ()

type Vector []int64

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
