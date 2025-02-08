package test

var ()

const ()

func Insert(p Point, e *Element) *Element {
	new := Element{Point: p}

	if e != nil {
		new.Next = e.Next
		new.Prev = e
		e.Next.Prev = &new
		e.Next = &new
	} else {
		new.Prev = &new
		new.Next = &new
	}
	return &new
}

type byMaxX [][]Point

type Point struct {
	X, Y float64
}

type Element struct {
	Prev, Next *Element
	Point      Point
}
