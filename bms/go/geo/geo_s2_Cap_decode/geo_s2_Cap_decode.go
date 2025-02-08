package test

import (
	"encoding/binary"
	"io"
	"math"
)

func (c *geo_s2_Cap) decode(d *geo_s2_decoder) {
	c.center.X = d.readFloat64()
	c.center.Y = d.readFloat64()
	c.center.Z = d.readFloat64()
	c.radius = geo_s1_ChordAngle(d.readFloat64())
}

func (d *geo_s2_decoder) readFloat64() float64 {
	if d.err != nil {
		return 0
	}
	buf := d.buffer()
	_, d.err = io.ReadFull(d.r, buf)
	return math.Float64frombits(binary.LittleEndian.Uint64(buf))
}

func (d *geo_s2_decoder) buffer() []byte {
	if d.buf == nil {
		d.buf = make([]byte, 8)
	}
	return d.buf
}

type geo_s2_Cap struct {
	center geo_s2_Point
	radius geo_s1_ChordAngle
}

type geo_s2_Point struct {
	geo_r3_Vector
}

type geo_r3_Vector struct {
	X, Y, Z float64
}

type geo_s1_ChordAngle float64

type geo_s2_decoder struct {
	r   geo_s2_byteReader // the real reader passed to Decode
	err error
	buf []byte
}

type geo_s2_byteReader interface {
	io.Reader
	io.ByteReader
}
