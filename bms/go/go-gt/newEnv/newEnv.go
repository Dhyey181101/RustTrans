package test

var ()

const ()

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

type Env struct {
	M, N int64

	T, S   []bool
	Slack  []int64
	Slackx []int64
	Prev   []int64
	Xy, Yx []int64
	Lx, Ly []int64
}
