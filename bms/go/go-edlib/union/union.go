package test

var ()

const ()

func union(a, b []string) [][]rune {
	m := make(map[string]bool)
	for _, item := range a {
		m[item] = true
	}
	for _, item := range b {
		if _, ok := m[item]; !ok {
			a = append(a, item)
		}
	}

	// Convert a to rune matrix (with x -> words and y -> characters)
	out := make([][]rune, len(a))
	for i, word := range a {
		out[i] = []rune(word)
	}
	return out
}
