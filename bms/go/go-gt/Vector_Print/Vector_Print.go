package test

import (
	"fmt"
)

var ()

const ()

type Vector []int64

func (v Vector) Print() {
	for i := 0; i < len(v); i++ {
		fmt.Printf("%d ", v[i])
	}
	fmt.Print("\n")
}
