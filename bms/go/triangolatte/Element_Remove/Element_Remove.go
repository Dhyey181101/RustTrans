package test 

import (
)

var (
)

const (
)

func (e *Element) Remove() {
	e.Next.Prev = e.Prev
	e.Prev.Next = e.Next
}

type Element struct {
	Prev, Next *Element
	
}

