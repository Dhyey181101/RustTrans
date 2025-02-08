package test

var ()

const ()

func (p OrderedMap) Len() int { return len(p) }

type pair struct {
	Key   string
	Value float32
}

// OrderedMap is a slice of pairs type with string keys and float values.
// It implement sorting methods by values.
type OrderedMap []pair
