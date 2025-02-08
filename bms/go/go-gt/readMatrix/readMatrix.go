package test

import (
	"bufio"
	"strconv"
)

var ()

const ()

func readMatrix(rd *bufio.Reader, n int64) *Matrix {
	M := NewMatrix(n)
	var i, j int64
	for i = 0; i < n; i++ {
		skip(rd)
		line, _ := rd.ReadString('\n')
		for j = 0; j < n; j++ {
			line = wskip(line)
			x, p := readUint(line)
			M.Set(j, i, x)
			if p == 0 {
				panic("bad int")
			}
			line = line[p:]
		}
	}
	return M
}

func NewMatrix(n int64) (m *Matrix) {
	m = new(Matrix)
	m.N = n
	m.A = make([]int64, n*n)
	return m
}

func skip(rd *bufio.Reader) {
	var b byte = ' '
	var err error
	for b == ' ' || b == '\t' || b == '\n' {
		b, err = rd.ReadByte()
		if err != nil {
			return
		}
	}
	rd.UnreadByte()
}

func wskip(s string) string {
	for i := 0; i < len(s); i++ {
		if s[i] != ' ' && s[i] != '\t' {
			return s[i:]
		}
	}
	return ""
}

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

func (m Matrix) Set(i int64, j int64, v int64) {
	m.A[i*m.N+j] = v
}

type bufio_Reader struct {

	// reader provided by the client
	// buf read and write positions

	// last byte read for UnreadByte; -1 means invalid
	// size of last rune read for UnreadRune; -1 means invalid
}

type Matrix struct {
	N int64
	A []int64
}
