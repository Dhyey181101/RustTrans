package test

import (
	"bufio"
	"encoding/binary"
	"io"
)

func (ci *geo_s2_CellID) Decode(r io.Reader) error {
	d := &geo_s2_decoder{r: geo_s2_asByteReader(r)}
	ci.decode(d)
	return d.err
}

func geo_s2_asByteReader(r io.Reader) geo_s2_byteReader {
	if br, ok := r.(geo_s2_byteReader); ok {
		return br
	}
	return bufio.NewReader(r)
}

func (ci *geo_s2_CellID) decode(d *geo_s2_decoder) {
	*ci = geo_s2_CellID(d.readUint64())
}

func (d *geo_s2_decoder) readUint64() (x uint64) {
	if d.err != nil {
		return
	}
	d.err = binary.Read(d.r, binary.LittleEndian, &x)
	return
}

type geo_s2_CellID uint64

type geo_s2_decoder struct {
	r   geo_s2_byteReader // the real reader passed to Decode
	err error
}

type geo_s2_byteReader interface {
	io.Reader
	io.ByteReader
}
