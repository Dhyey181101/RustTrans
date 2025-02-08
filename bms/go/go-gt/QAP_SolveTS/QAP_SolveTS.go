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

func QAP_SolveTS(a, b *Matrix, p Vector, opt, tabu_duration, aspiration, nr_iterations int64) int64 {
	var i, j, current_cost, iter int64
	best_cost := Inf
	n := p.Len()
	dist := NewMatrix(n)
	tabu_list := NewMatrix(n)
	best_sol := make(Vector, n)
	best_sol.Copy(p)
	current_cost = 0
	for i = 0; i < n; i++ {
		for j = 0; j < n; j++ {
			current_cost += a.Get(i, j) * b.Get(p[i], p[j])
			if i < j {
				dist.Set(i, j, delta(a, b, p, i, j))
			}
		}
	}
	best_cost = current_cost

	// tabu list initialization
	for i = 0; i < n; i++ {
		for j = 0; j < n; j++ {
			tabu_list.Set(i, j, -(n*i + j))
		}
	}

	// tabu search loop
	for iter = 0; iter < nr_iterations && best_cost > opt; iter++ {
		// find best move (i_retained, j_retained)
		i_retained := Inf // in case all moves are tabu
		j_retained := Inf
		min_dist := Inf
		already_aspired := false

		for i = 0; i < n-1; i++ {
			for j = i + 1; j < n; j++ {
				autorized := (tabu_list.Get(i, p[j]) < iter) ||
					(tabu_list.Get(j, p[i]) < iter)

				aspired :=
					(tabu_list.Get(i, p[j]) < iter-aspiration) ||
						(tabu_list.Get(j, p[i]) < iter-aspiration) ||
						(current_cost+dist.Get(i, j) < best_cost)

				if (aspired && !already_aspired) || // first move aspired
					(aspired && already_aspired && // many move aspired
						(dist.Get(i, j) < min_dist)) || // => take best one
					(!aspired && !already_aspired && // no move aspired yet
						(dist.Get(i, j) < min_dist) && autorized) {
					i_retained = i
					j_retained = j
					min_dist = dist.Get(i, j)
					if aspired {
						already_aspired = true
					}
				}
			}
		}

		if i_retained == Inf {
			fmt.Println("All moves are tabu!") // to be improved
		} else { // transpose elements in pos. i_retained and j_retained

			p.Swap(i_retained, j_retained)

			// update solution value
			current_cost += dist.Get(i_retained, j_retained)
			// forbid reverse move for a random number of iterations
			z := iter + int64(cube(rand.Float64()))*tabu_duration
			tabu_list.Set(i_retained, p[j_retained], z)
			z = iter + int64(cube(rand.Float64()))*tabu_duration
			tabu_list.Set(j_retained, p[i_retained], z)

			// best solution improved ?
			if current_cost < best_cost {
				best_cost = current_cost
				best_sol.Copy(p)
				if Verbose {
					fmt.Printf("iteration %d: cost=%d\n", iter, best_cost)
					best_sol.Print()
				}
			}

			// update matrix of the move costs
			for i = 0; i < n-1; i = i + 1 {
				for j = i + 1; j < n; j = j + 1 {
					if i != i_retained && i != j_retained &&
						j != i_retained && j != j_retained {
						y := delta_part(a, b, dist, p, i, j, i_retained, j_retained)
						dist.Set(i, j, y)
					} else {
						y := delta(a, b, p, i, j)
						dist.Set(i, j, y)
					}
				}
			}
		}
	}
	p.Copy(best_sol)
	return best_cost
}

func (v Vector) Len() int64 {
	return int64(len(v))
}

func NewMatrix(n int64) (m *Matrix) {
	m = new(Matrix)
	m.N = n
	m.A = make([]int64, n*n)
	return m
}

func (v Vector) Copy(w Vector) {
	for i := 0; i < len(v); i++ {
		v[i] = w[i]
	}
}

func (m Matrix) Get(i int64, j int64) int64 {
	return m.A[i*m.N+j]
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

func (m Matrix) Set(i int64, j int64, v int64) {
	m.A[i*m.N+j] = v
}

func (p Vector) Swap(i int64, j int64) {
	x := p[i]
	p[i] = p[j]
	p[j] = x
}

func cube(x float64) float64 {
	return x * x * x
}

func (v Vector) Print() {
	for i := 0; i < len(v); i++ {
		fmt.Printf("%d ", v[i])
	}
	fmt.Print("\n")
}

func delta_part(a, b, dist *Matrix, p Vector, i, j, r, s int64) int64 {
	return (dist.Get(i, j) + (a.Get(r, i)-a.Get(r, j)+a.Get(s, j)-a.Get(s, i))*
		(b.Get(p[s], p[i])-b.Get(p[s], p[j])+b.Get(p[r], p[j])-b.Get(p[r], p[i])) +
		(a.Get(i, r)-a.Get(j, r)+a.Get(j, s)-a.Get(i, s))*
			(b.Get(p[i], p[s])-b.Get(p[j], p[s])+b.Get(p[j], p[r])-b.Get(p[i], p[r])))
}

type Vector []int64

type Matrix struct {
	N int64
	A []int64
}
