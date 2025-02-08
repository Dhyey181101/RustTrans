package test

import (
	"math"
)

var ()

const ()

func Hungarian(g *Matrix) (xy []int64, yx []int64) {
	e := initH(g)
	e.augment()
	return e.Xy, e.Yx
}

func initH(g *Matrix) (e *Env) {
	var i, j int64
	e = newEnv(g.N)
	e.G = g
	e.N = g.N
	for i = 0; i < e.N; i++ {
		for j = 0; j < e.N; j++ {
			e.Lx[i] = max(e.Lx[i], e.G.Get(i, j))
		}
	}
	return e
}

func newEnv(n int64) *Env {
	e := new(Env)
	e.M = 0
	e.N = n
	e.T = make([]bool, n)
	e.S = make([]bool, n)
	e.Slack = make([]int64, n)
	e.Slackx = make([]int64, n)
	e.Prev = make([]int64, n)
	e.Xy = make([]int64, n)
	e.Yx = make([]int64, n)
	e.Lx = make([]int64, n)
	e.Ly = make([]int64, n)
	var i int64
	for i = 0; i < n; i++ {
		e.Xy[i] = -1
		e.Yx[i] = -1
	}
	return e
}

func (m Matrix) Get(i int64, j int64) int64 {
	return m.A[i*m.N+j]
}

func max(a, b int64) int64 {
	if a > b {
		return a
	}
	return b
}

func (e *Env) augment() {
	var i, j, wr, rd, r int64
	wr = 0
	rd = 0
	q := make([]int64, e.N)
	if e.M == e.N {
		return
	}
	for i = 0; i < e.N; i++ {
		if e.Xy[i] == -1 {
			wr++
			q[wr] = i
			r = i
			e.Prev[i] = -2
			e.S[i] = true
			break
		}
	}
	for i = 0; i < e.N; i++ {
		e.Slack[i] = e.Lx[r] + e.Ly[i] - e.G.Get(r, i)
		e.Slackx[i] = r
	}
	for {
		for rd < wr {
			rd++
			i = q[rd]
			for j = 0; j < e.N; j++ {
				if e.G.Get(i, j) == e.Lx[i]+e.Ly[j] && !e.T[j] {
					if e.Yx[j] == -1 {
						break
					}
					e.T[j] = true
					wr++
					q[wr] = e.Yx[j]
					e.add(e.Yx[j], i)
				}
			}
			if j < e.N {
				break
			}
		}
		if j < e.N {
			break
		}
		e.update()
		wr = 0
		rd = 0
		for j = 0; j < e.N; j++ {
			if !e.T[j] && e.Slack[j] == 0 {
				if e.Yx[i] == -1 {
					i = e.Slackx[j]
					break
				} else {
					e.T[j] = true
					if !e.S[e.Yx[j]] {
						wr++
						q[wr] = e.Yx[j]
						e.add(e.Yx[j], e.Slackx[j])
					}
				}
			}
		}
		if j < e.N {
			return
		}
	}
	if j < e.N {
		e.M++
		for i != -2 {
			k := e.Xy[i]
			e.Yx[j] = i
			e.Xy[i] = j
			i = e.Prev[i]
			j = k
		}
		e.augment()
	}
}

func (e *Env) add(i, p int64) {
	var j int64
	e.S[i] = true
	e.Prev[i] = p
	for j = 0; j < e.N; j++ {
		if e.Lx[i]+e.Ly[i]-e.G.Get(i, j) < e.Slack[i] {
			e.Slack[i] = e.Lx[i] + e.Ly[i] - e.G.Get(i, j)
			e.Slackx[i] = j
		}
	}
}

func (e *Env) update() {
	var i int64
	var d int64 = math.MaxInt64
	for i = 0; i < e.N; i++ {
		if !e.T[i] {
			d = min(d, e.Slack[i])
		}
	}
	for i = 0; i < e.N; i++ {
		if e.S[i] {
			e.Lx[i] -= d
		}
	}
	for i = 0; i < e.N; i++ {
		if e.T[i] {
			e.Ly[i] += d
		}
	}
	for i = 0; i < e.N; i++ {
		if !e.T[i] {
			e.Slack[i] -= d
		}
	}
}

func min(a, b int64) int64 {
	if a < b {
		return a
	}
	return b
}

type Matrix struct {
	N int64
	A []int64
}

type Env struct {
	M, N   int64
	G      *Matrix
	T, S   []bool
	Slack  []int64
	Slackx []int64
	Prev   []int64
	Xy, Yx []int64
	Lx, Ly []int64
}
