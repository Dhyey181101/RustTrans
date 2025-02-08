package test

var ()

const ()

type Vector []int64

func (v Vector) Copy(w Vector) {
	for i := 0; i < len(v); i++ {
		v[i] = w[i]
	}
}
