package test

import (
	"strings"
	"unicode/utf8"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (Addenda99 *moov_io_ach_Addenda99) IATPaymentAmount(s string) {
	Addenda99.AddendaInformation = Addenda99.stringField(s, 10)
}

func (c *moov_io_ach_converters) stringField(s string, max uint) string {
	ln := uint(utf8.RuneCountInString(s))
	if ln > max {
		return s[:max]
	}

	// Pad with preallocated string
	m := int(max - ln)
	pad, exists := moov_io_ach_stringZeros[m]
	if exists {
		return pad + s
	}
	// slow path
	return strings.Repeat("0", m) + s
}

func moov_io_ach_populateMap(max int, zero string) map[int]string {
	out := make(map[int]string, max)
	for i := 0; i < max; i++ {
		out[i] = strings.Repeat(zero, i)
	}
	return out
}

type moov_io_ach_Addenda99 struct {
	// ID is a client defined string used as a reference to this record.

	// TypeCode Addenda types code '99'

	// ReturnCode field contains a standard code used by an ACH Operator or RDFI to describe the reason for returning an Entry.
	// Must exist in returnCodeDict

	// OriginalTrace This field contains the Trace Number as originally included on the forward Entry or Prenotification.
	// The RDFI must include the Original Entry Trace Number in the Addenda Record of an Entry being returned to an ODFI,
	// in the Addenda Record of an 98, within an Acknowledgment Entry, or with an RDFI request for a copy of an authorization.

	// DateOfDeath The field date of death is to be supplied on Entries being returned for reason of death (return reason codes R14 and R15). Format: YYMMDD (Y=Year, M=Month, D=Day)

	// OriginalDFI field contains the Receiving DFI Identification (addenda.RDFIIdentification) as originally included on the forward Entry or Prenotification that the RDFI is returning or correcting.

	// AddendaInformation
	AddendaInformation string `json:"addendaInformation,omitempty"`
	// TraceNumber matches the Entry Detail Trace Number of the entry being returned.
	//
	// Use TraceNumberField for a properly formatted string representation.

	// validator is composed for data validation

	// converters is composed for ACH to GoLang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
