package test

import (
	"strconv"
)

var ()

const ()

func readUint(s string) (int64, int64) {
	i := end(s)
	x, _ := strconv.ParseInt(s[:i], 10, 64)
	return int64(x), i
}

func end(s string) (i int64) {
	for i = 0; i < int64(len(s)); i++ {
		if s[i] == ' ' || s[i] == '\t' || s[i] == '\n' {
			return i
		}
	}
	return 0
}
