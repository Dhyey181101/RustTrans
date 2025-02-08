package test

var ()

const ()

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
