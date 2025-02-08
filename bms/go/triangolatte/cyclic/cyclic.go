package test

var ()

const ()

func cyclic(i, n int) int {
	return (i%n + n) % n
}
