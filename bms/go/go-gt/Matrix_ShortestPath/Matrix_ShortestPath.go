package test

import "container/list"

var ()

const ()

func (G *Matrix) ShortestPath(src, tar int64, N *Matrix) (p *list.List) {
	p = list.New()
	if G.Get(src, tar) == 0 {
		return
	}
	next := N.Get(src, tar)
	if next == 0 {
		p.PushBack(tar)
	} else {
		p.PushBackList(G.ShortestPath(src, next-1, N))
		p.PushBackList(G.ShortestPath(next-1, tar, N))
	}
	return
}

func (m Matrix) Get(i int64, j int64) int64 {
	return m.A[i*m.N+j]
}

type Matrix struct {
	N int64
	A []int64
}

type container_list_List struct {
	// sentinel list element, only &root, root.prev, and root.next are used
	// current list length excluding (this) sentinel element
}

type container_list_Element struct {
	// Next and previous pointers in the doubly-linked list of elements.
	// To simplify the implementation, internally a list l is implemented
	// as a ring, such that &l.root is both the next element of the last
	// list element (l.Back()) and the previous element of the first list
	// element (l.Front()).

	// The list to which this element belongs.

	// The value stored with this element.

}
