package test

var ()

const ()

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

func (m Matrix) Get(i int64, j int64) int64 {
	return m.A[i*m.N+j]
}

type Env struct {
	N int64
	G *Matrix

	S []bool

	Slack  []int64
	Slackx []int64
	Prev   []int64

	Lx, Ly []int64
}

type Matrix struct {
	N int64
	A []int64
}
