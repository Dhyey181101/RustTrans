package test

import (
	"encoding/binary"
	"io"
)

func (c geo_s2_Cap) Encode(w io.Writer) error {
	e := &geo_s2_encoder{w: w}
	c.encode(e)
	return e.err
}

func (c geo_s2_Cap) encode(e *geo_s2_encoder) {
	e.writeFloat64(c.center.X)
	e.writeFloat64(c.center.Y)
	e.writeFloat64(c.center.Z)
	e.writeFloat64(float64(c.radius))
}

func (e *geo_s2_encoder) writeFloat64(x float64) {
	if e.err != nil {
		return
	}
	e.err = binary.Write(e.w, binary.LittleEndian, x)
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

type geo_s2_encoder struct {
	w   io.Writer // the real writer passed to Encode
	err error
}
