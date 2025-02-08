package test

var ()

const ()

func end(s string) (i int64) {
	for i = 0; i < int64(len(s)); i++ {
		if s[i] == ' ' || s[i] == '\t' || s[i] == '\n' {
			return i
		}
	}
	return 0
}
