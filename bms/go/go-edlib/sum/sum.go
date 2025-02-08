package test

var ()

const ()

func sum(arr []int) int {
	var res int
	for _, v := range arr {
		res += v
	}
	return res
}
