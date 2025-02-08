package test

import (
	"fmt"
)

var ()

const ()

func (m *Matrix) Print() {
	var i, j int64
	for i = 0; i < m.N; i++ {
		for j = 0; j < m.N; j++ {
			fmt.Printf("%d ", m.Get(i, j))
		}
		fmt.Print("\n")
	}
}

func (m Matrix) Get(i int64, j int64) int64 {
	return m.A[i*m.N+j]
}

type Matrix struct {
	N int64
	A []int64
}
