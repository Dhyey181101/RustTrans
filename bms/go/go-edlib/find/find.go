package test

var ()

const ()

func find(slice [][]rune, val []rune) int {
	for i, item := range slice {
		if Equal(item, val) {
			return i
		}
	}
	return -1
}

func Equal(a, b []rune) bool {
	if len(a) != len(b) {
		return false
	}
	for i, v := range a {
		if v != b[i] {
			return false
		}
	}
	return true
}
