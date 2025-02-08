package test

import (
	"strconv"
	"strings"
	"unicode/utf8"
)

var (
	moov_io_ach_spaceZeros  map[int]string = moov_io_ach_populateMap(94, " ")
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const (
	moov_io_ach_RecordLength    = 94
	moov_io_ach_entryAddendaPos = "7"
)

func (addenda15 *moov_io_ach_Addenda15) String() string {
	if addenda15 == nil {
		return ""
	}

	var buf strings.Builder
	buf.Grow(94)
	buf.WriteString(moov_io_ach_entryAddendaPos)
	buf.WriteString(addenda15.TypeCode)
	buf.WriteString(addenda15.ReceiverIDNumberField())
	buf.WriteString(addenda15.ReceiverStreetAddressField())
	buf.WriteString("                                  ")
	buf.WriteString(addenda15.EntryDetailSequenceNumberField())
	return buf.String()
}

func (addenda15 *moov_io_ach_Addenda15) ReceiverIDNumberField() string {
	return addenda15.alphaField(addenda15.ReceiverIDNumber, 15)
}

func (c *moov_io_ach_converters) alphaField(s string, max uint) string {
	ln := uint(utf8.RuneCountInString(s))
	if ln > max {
		return s[:max]
	}

	m := int(max - ln)
	pad, exists := moov_io_ach_spaceZeros[m]
	if exists {
		return s + pad
	}
	// slow path
	return s + strings.Repeat(" ", int(max-ln))
}

func (addenda15 *moov_io_ach_Addenda15) ReceiverStreetAddressField() string {
	return addenda15.alphaField(addenda15.ReceiverStreetAddress, 35)
}

func (addenda15 *moov_io_ach_Addenda15) EntryDetailSequenceNumberField() string {
	return addenda15.numericField(addenda15.EntryDetailSequenceNumber, 7)
}

func (c *moov_io_ach_converters) numericField(n int, max uint) string {
	s := strconv.FormatInt(int64(n), 10)
	if l := uint(len(s)); l > max {
		return s[l-max:]
	} else {
		m := int(max - l)
		pad, exists := moov_io_ach_stringZeros[m]
		if exists {
			return pad + s
		}
		// slow path
		return strings.Repeat("0", m) + s
	}
}

func moov_io_ach_populateMap(max int, zero string) map[int]string {
	out := make(map[int]string, max)
	for i := 0; i < max; i++ {
		out[i] = strings.Repeat(zero, i)
	}
	return out
}

type moov_io_ach_Addenda15 struct {
	// ID is a client defined string used as a reference to this record.

	// TypeCode Addenda15 types code '15'
	TypeCode string `json:"typeCode"`
	// Receiver Identification Number contains the accounting number by which the Originator is known to
	// the Receiver for descriptive purposes. NACHA Rules recommend but do not require the RDFI to print
	// the contents of this field on the receiver's statement.
	ReceiverIDNumber string `json:"receiverIDNumber,omitempty"`
	// Receiver Street Address contains the Receiver's physical address
	ReceiverStreetAddress string `json:"receiverStreetAddress"`
	// EntryDetailSequenceNumber contains the ascending sequence number section of the Entry
	// Detail or Corporate Entry Detail Record's trace number This number is
	// the same as the last seven digits of the trace number of the related
	// Entry Detail Record or Corporate Entry Detail Record.
	EntryDetailSequenceNumber int `json:"entryDetailSequenceNumber"`
	// validator is composed for data validation

	// converters is composed for ACH to GoLang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
