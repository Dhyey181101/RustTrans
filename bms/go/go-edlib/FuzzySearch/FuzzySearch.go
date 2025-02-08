package test

import (
	"errors"
	"math"
	"strings"
)

var ()

// Algorithm identifiers
const (
	Levenshtein Algorithm = iota
	DamerauLevenshtein
	OSADamerauLevenshtein
	Lcs
	Hamming
	Jaro
	JaroWinkler
	Cosine
	Jaccard
	SorensenDice
	Qgram
)

func FuzzySearch(str string, strList []string, algo Algorithm) (string, error) {
	var higherMatchPercent float32
	var tmpStr string
	for _, strToCmp := range strList {
		sim, err := StringsSimilarity(str, strToCmp, algo)
		if err != nil {
			return "", err
		}

		if sim == 1.0 {
			return strToCmp, nil
		} else if sim > higherMatchPercent {
			higherMatchPercent = sim
			tmpStr = strToCmp
		}
	}

	return tmpStr, nil
}

func StringsSimilarity(str1 string, str2 string, algo Algorithm) (float32, error) {
	switch algo {
	case Levenshtein:
		return matchingIndex(str1, str2, LevenshteinDistance(str1, str2)), nil
	case DamerauLevenshtein:
		return matchingIndex(str1, str2, DamerauLevenshteinDistance(str1, str2)), nil
	case OSADamerauLevenshtein:
		return matchingIndex(str1, str2, OSADamerauLevenshteinDistance(str1, str2)), nil
	case Lcs:
		return matchingIndex(str1, str2, LCSEditDistance(str1, str2)), nil
	case Hamming:
		distance, err := HammingDistance(str1, str2)
		if err == nil {
			return matchingIndex(str1, str2, distance), nil
		}
		return 0.0, err
	case Jaro:
		return JaroSimilarity(str1, str2), nil
	case JaroWinkler:
		return JaroWinklerSimilarity(str1, str2), nil
	case Cosine:
		return CosineSimilarity(str1, str2, 2), nil
	case Jaccard:
		return JaccardSimilarity(str1, str2, 2), nil
	case SorensenDice:
		return SorensenDiceCoefficient(str1, str2, 2), nil
	case Qgram:
		return QgramSimilarity(str1, str2, 2), nil
	default:
		return 0.0, errors.New("Illegal argument for algorithm method")
	}
}

func LevenshteinDistance(str1, str2 string) int {
	// Convert string parameters to rune arrays to be compatible with non-ASCII
	runeStr1 := []rune(str1)
	runeStr2 := []rune(str2)

	// Get and store length of these strings
	runeStr1len := len(runeStr1)
	runeStr2len := len(runeStr2)
	if runeStr1len == 0 {
		return runeStr2len
	} else if runeStr2len == 0 {
		return runeStr1len
	} else if Equal(runeStr1, runeStr2) {
		return 0
	}

	column := make([]int, runeStr1len+1)

	for y := 1; y <= runeStr1len; y++ {
		column[y] = y
	}
	for x := 1; x <= runeStr2len; x++ {
		column[0] = x
		lastkey := x - 1
		for y := 1; y <= runeStr1len; y++ {
			oldkey := column[y]
			var i int
			if runeStr1[y-1] != runeStr2[x-1] {
				i = 1
			}
			column[y] = Min(
				Min(column[y]+1, // insert
					column[y-1]+1), // delete
				lastkey+i) // substitution
			lastkey = oldkey
		}
	}

	return column[runeStr1len]
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

func Min(a int, b int) int {
	if b < a {
		return b
	}
	return a
}

func matchingIndex(str1 string, str2 string, distance int) float32 {
	// Convert strings to rune slices
	runeStr1 := []rune(str1)
	runeStr2 := []rune(str2)
	// Compare rune arrays length and make a matching percentage between them
	if len(runeStr1) >= len(runeStr2) {
		return float32(len(runeStr1)-distance) / float32(len(runeStr1))
	}
	return float32(len(runeStr2)-distance) / float32(len(runeStr2))
}

func DamerauLevenshteinDistance(str1, str2 string) int {
	// Convert string parameters to rune arrays to be compatible with non-ASCII
	runeStr1 := []rune(str1)
	runeStr2 := []rune(str2)

	// Get and store length of these strings
	runeStr1len := len(runeStr1)
	runeStr2len := len(runeStr2)
	if runeStr1len == 0 {
		return runeStr2len
	} else if runeStr2len == 0 {
		return runeStr1len
	} else if Equal(runeStr1, runeStr2) {
		return 0
	}

	// Create alphabet based on input strings
	da := make(map[rune]int)
	for i := 0; i < runeStr1len; i++ {
		da[runeStr1[i]] = 0
	}
	for i := 0; i < runeStr2len; i++ {
		da[runeStr2[i]] = 0
	}

	// 2D Array for distance matrix : matrix[0..str1.length+2][0..s2.length+2]
	matrix := make([][]int, runeStr1len+2)
	for i := 0; i <= runeStr1len+1; i++ {
		matrix[i] = make([]int, runeStr2len+2)
		for j := 0; j <= runeStr2len+1; j++ {
			matrix[i][j] = 0
		}
	}

	// Maximum possible distance
	maxDist := runeStr1len + runeStr2len

	// Initialize matrix
	matrix[0][0] = maxDist
	for i := 0; i <= runeStr1len; i++ {
		matrix[i+1][0] = maxDist
		matrix[i+1][1] = i
	}
	for i := 0; i <= runeStr2len; i++ {
		matrix[0][i+1] = maxDist
		matrix[1][i+1] = i
	}

	// Process edit distance
	var cost int
	for i := 1; i <= runeStr1len; i++ {
		db := 0
		for j := 1; j <= runeStr2len; j++ {
			i1 := da[runeStr2[j-1]]
			j1 := db
			if runeStr1[i-1] == runeStr2[j-1] {
				cost = 0
				db = j
			} else {
				cost = 1
			}

			matrix[i+1][j+1] = Min(
				Min(
					matrix[i+1][j]+1,  // Addition
					matrix[i][j+1]+1), // Deletion
				Min(
					matrix[i][j]+cost, // Substitution
					matrix[i1][j1]+(i-i1-1)+1+(j-j1-1))) // Transposition
		}

		da[runeStr1[i-1]] = i
	}

	return matrix[runeStr1len+1][runeStr2len+1]
}

func OSADamerauLevenshteinDistance(str1, str2 string) int {
	// Convert string parameters to rune arrays to be compatible with non-ASCII
	runeStr1 := []rune(str1)
	runeStr2 := []rune(str2)

	// Get and store length of these strings
	runeStr1len := len(runeStr1)
	runeStr2len := len(runeStr2)
	if runeStr1len == 0 {
		return runeStr2len
	} else if runeStr2len == 0 {
		return runeStr1len
	} else if Equal(runeStr1, runeStr2) {
		return 0
	} else if runeStr1len < runeStr2len {
		return OSADamerauLevenshteinDistance(str2, str1)
	}

	// 2D Array
	row := Min(runeStr1len+1, 3)
	matrix := make([][]int, row)
	for i := 0; i < row; i++ {
		matrix[i] = make([]int, runeStr2len+1)
		matrix[i][0] = i
	}

	for j := 0; j <= runeStr2len; j++ {
		matrix[0][j] = j
	}

	var count int
	for i := 1; i <= runeStr1len; i++ {
		matrix[i%3][0] = i
		for j := 1; j <= runeStr2len; j++ {
			if runeStr1[i-1] == runeStr2[j-1] {
				count = 0
			} else {
				count = 1
			}

			matrix[i%3][j] = Min(Min(matrix[(i-1)%3][j]+1, matrix[i%3][j-1]+1),
				matrix[(i-1)%3][j-1]+count) // insertion, deletion, substitution
			if i > 1 && j > 1 && runeStr1[i-1] == runeStr2[j-2] && runeStr1[i-2] == runeStr2[j-1] {
				matrix[i%3][j] = Min(matrix[i%3][j], matrix[(i-2)%3][j-2]+1) // translation
			}
		}
	}
	return matrix[runeStr1len%3][runeStr2len]
}

func LCSEditDistance(str1, str2 string) int {
	if len(str1) == 0 {
		return len(str2)
	} else if len(str2) == 0 {
		return len(str1)
	} else if str1 == str2 {
		return 0
	}

	lcs := LCS(str1, str2)
	return (len([]rune(str1)) - lcs) + (len([]rune(str2)) - lcs)
}

func LCS(str1, str2 string) int {
	// Convert strings to rune array to handle no-ASCII characters
	runeStr1 := []rune(str1)
	runeStr2 := []rune(str2)

	if len(runeStr1) == 0 || len(runeStr2) == 0 {
		return 0
	} else if Equal(runeStr1, runeStr2) {
		return len(runeStr1)
	}

	lcsMatrix := lcsProcess(runeStr1, runeStr2)
	return lcsMatrix[len(runeStr1)][len(runeStr2)]
}

func lcsProcess(runeStr1, runeStr2 []rune) [][]int {
	// 2D Array that will contain str1 and str2 LCS
	lcsMatrix := make([][]int, len(runeStr1)+1)
	for i := 0; i <= len(runeStr1); i++ {
		lcsMatrix[i] = make([]int, len(runeStr2)+1)
		for j := 0; j <= len(runeStr2); j++ {
			lcsMatrix[i][j] = 0
		}
	}

	for i := 1; i <= len(runeStr1); i++ {
		for j := 1; j <= len(runeStr2); j++ {
			if runeStr1[i-1] == runeStr2[j-1] {
				lcsMatrix[i][j] = lcsMatrix[i-1][j-1] + 1
			} else {
				lcsMatrix[i][j] = Max(lcsMatrix[i][j-1], lcsMatrix[i-1][j])
			}
		}
	}

	return lcsMatrix
}

func Max(a int, b int) int {
	if b > a {
		return b
	}
	return a
}

func HammingDistance(str1, str2 string) (int, error) {
	// Convert strings to rune array to handle no-ASCII characters
	runeStr1 := []rune(str1)
	runeStr2 := []rune(str2)

	if len(runeStr1) != len(runeStr2) {
		return 0, errors.New("Undefined for strings of unequal length")
	} else if Equal(runeStr1, runeStr2) {
		return 0, nil
	}

	var counter int
	for i := 0; i < len(runeStr1); i++ {
		if runeStr1[i] != runeStr2[i] {
			counter++
		}
	}

	return counter, nil
}

func JaroSimilarity(str1, str2 string) float32 {
	// Convert string parameters to rune arrays to be compatible with non-ASCII
	runeStr1 := []rune(str1)
	runeStr2 := []rune(str2)

	// Get and store length of these strings
	runeStr1len := len(runeStr1)
	runeStr2len := len(runeStr2)
	if runeStr1len == 0 || runeStr2len == 0 {
		return 0.0
	} else if Equal(runeStr1, runeStr2) {
		return 1.0
	}

	var match int
	// Maximum matching distance allowed
	maxDist := Max(runeStr1len, runeStr2len)/2 - 1
	// Correspondence tables (1 for matching and 0 if it's not the case)
	str1Table := make([]int, runeStr1len)
	str2Table := make([]int, runeStr2len)

	// Check for matching characters in both strings
	for i := 0; i < runeStr1len; i++ {
		for j := Max(0, i-maxDist); j < Min(runeStr2len, i+maxDist+1); j++ {
			if runeStr1[i] == runeStr2[j] && str2Table[j] == 0 {
				str1Table[i] = 1
				str2Table[j] = 1
				match++
				break
			}
		}
	}
	if match == 0 {
		return 0.0
	}

	var t float32
	var p int
	// Check for possible translations
	for i := 0; i < runeStr1len; i++ {
		if str1Table[i] == 1 {
			for str2Table[p] == 0 {
				p++
			}
			if runeStr1[i] != runeStr2[p] {
				t++
			}
			p++
		}
	}
	t /= 2

	return (float32(match)/float32(runeStr1len) +
		float32(match)/float32(runeStr2len) +
		(float32(match)-t)/float32(match)) / 3.0
}

func JaroWinklerSimilarity(str1, str2 string) float32 {
	// Get Jaro similarity index between str1 and str2
	jaroSim := JaroSimilarity(str1, str2)

	if jaroSim != 0.0 && jaroSim != 1.0 {
		// Convert string parameters to rune arrays to be compatible with non-ASCII
		runeStr1 := []rune(str1)
		runeStr2 := []rune(str2)

		// Get and store length of these strings
		runeStr1len := len(runeStr1)
		runeStr2len := len(runeStr2)

		var prefix int

		// Find length of the common prefix
		for i := 0; i < Min(runeStr1len, runeStr2len); i++ {
			if runeStr1[i] == runeStr2[i] {
				prefix++
			} else {
				break
			}
		}

		// Normalized prefix count with Winkler's constraint
		// (prefix length must be inferior or equal to 4)
		prefix = Min(prefix, 4)

		// Return calculated Jaro-Winkler similarity index
		return jaroSim + 0.1*float32(prefix)*(1-jaroSim)
	}

	return jaroSim
}

func CosineSimilarity(str1, str2 string, splitLength int) float32 {
	if str1 == "" || str2 == "" {
		return 0
	}

	// Split string before rune conversion for cosine calculation
	// If splitLength == 0 then split on whitespaces
	// Else use shingle algorithm
	var splittedStr1, splittedStr2 []string
	if splitLength == 0 {
		splittedStr1 = strings.Split(str1, " ")
		splittedStr2 = strings.Split(str2, " ")
	} else {
		splittedStr1 = ShingleSlice(str1, splitLength)
		splittedStr2 = ShingleSlice(str2, splitLength)
	}

	// Conversion of plitted string into rune array
	runeStr1 := make([][]rune, len(splittedStr1))
	for i, str := range splittedStr1 {
		runeStr1[i] = []rune(str)
	}
	runeStr2 := make([][]rune, len(splittedStr2))
	for i, str := range splittedStr2 {
		runeStr2[i] = []rune(str)
	}

	var l1, l2 []int
	// Create union keywords slice between input strings
	unionStr := union(splittedStr1, splittedStr2)
	for _, word := range unionStr {
		fw := find(runeStr1, word)
		if fw != -1 {
			l1 = append(l1, 1)
		} else {
			l1 = append(l1, 0)
		}

		fw = find(runeStr2, word)
		if fw != -1 {
			l2 = append(l2, 1)
		} else {
			l2 = append(l2, 0)
		}
	}

	// Compute cosine algorithm
	var cosineSim float32
	for i := 0; i < len(unionStr); i++ {
		cosineSim += float32(l1[i] * l2[i])
	}

	return cosineSim / float32(math.Sqrt(float64(sum(l1)*sum(l2))))
}

func ShingleSlice(s string, k int) []string {
	var out []string
	m := make(map[string]int)
	if s != "" && k != 0 {
		runeS := []rune(s)
		for i := 0; i < len(runeS)-k+1; i++ {
			m[string(runeS[i:i+k])]++
		}
		for k := range m {
			out = append(out, k)
		}
	}
	return out
}

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

func find(slice [][]rune, val []rune) int {
	for i, item := range slice {
		if Equal(item, val) {
			return i
		}
	}
	return -1
}

func sum(arr []int) int {
	var res int
	for _, v := range arr {
		res += v
	}
	return res
}

func JaccardSimilarity(str1, str2 string, splitLength int) float32 {
	if str1 == "" || str2 == "" {
		return 0
	}

	// Split string before rune conversion for jaccard calculation
	// If splitLength == 0 then split on whitespaces
	// Else use shingle algorithm
	var splittedStr1, splittedStr2 []string
	if splitLength == 0 {
		splittedStr1 = strings.Split(str1, " ")
		splittedStr2 = strings.Split(str2, " ")
	} else {
		splittedStr1 = ShingleSlice(str1, splitLength)
		splittedStr2 = ShingleSlice(str2, splitLength)
	}

	// Conversion of splitted string into rune array
	runeStr1 := make([][]rune, len(splittedStr1))
	for i, str := range splittedStr1 {
		runeStr1[i] = []rune(str)
	}
	runeStr2 := make([][]rune, len(splittedStr2))
	for i, str := range splittedStr2 {
		runeStr2[i] = []rune(str)
	}

	// Create union keywords slice between input strings
	unionStr := union(splittedStr1, splittedStr2)
	jacc := float32(len(runeStr1) + len(runeStr2) - len(unionStr))

	return jacc / float32(len(unionStr))
}

func SorensenDiceCoefficient(str1, str2 string, splitLength int) float32 {
	if str1 == "" && str2 == "" {
		return 0
	}
	shingle1 := Shingle(str1, splitLength)
	shingle2 := Shingle(str2, splitLength)

	intersection := float32(0)
	for i := range shingle1 {
		if _, ok := shingle2[i]; ok {
			intersection++
		}
	}
	return 2.0 * intersection / float32(len(shingle1)+len(shingle2))
}

func Shingle(s string, k int) map[string]int {
	m := make(map[string]int)
	if s != "" && k != 0 {
		runeS := []rune(s)

		for i := 0; i < len(runeS)-k+1; i++ {
			m[string(runeS[i:i+k])]++
		}
	}
	return m
}

func QgramSimilarity(str1, str2 string, splitLength int) float32 {
	splittedStr1 := Shingle(str1, splitLength)
	splittedStr2 := Shingle(str2, splitLength)
	res := float32(QgramDistanceCustomNgram(splittedStr1, splittedStr2))
	return 1 - (res / float32(len(splittedStr1)+len(splittedStr2)))
}

func QgramDistanceCustomNgram(splittedStr1, splittedStr2 map[string]int) int {
	union := make(map[string]int)
	for i := range splittedStr1 {
		union[i] = 0
	}
	for i := range splittedStr2 {
		union[i] = 0
	}

	res := 0
	for i := range union {
		res += int(math.Abs(float64(splittedStr1[i] - splittedStr2[i])))
	}

	return res
}

type Algorithm uint8
