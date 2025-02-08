package test

var ()

const ()

func (m StringHashMap) ToArray() []string {
	var index int
	arr := make([]string, 0, len(m))
	for key := range m {
		arr = append(arr, key)
		index++
	}

	return arr
}

// StringHashMap is HashMap substitute for string
type StringHashMap map[string]struct{}
