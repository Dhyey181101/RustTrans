package test

import (
	"encoding/binary"
	"io"
)

func (ci geo_s2_CellID) Encode(w io.Writer) error {
	e := &geo_s2_encoder{w: w}
	ci.encode(e)
	return e.err
}

func (ci geo_s2_CellID) encode(e *geo_s2_encoder) {
	e.writeUint64(uint64(ci))
}

func (e *geo_s2_encoder) writeUint64(x uint64) {
	if e.err != nil {
		return
	}
	e.err = binary.Write(e.w, binary.LittleEndian, x)
}

type geo_s2_CellID uint64

type geo_s2_encoder struct {
	w   io.Writer // the real writer passed to Encode
	err error
}
