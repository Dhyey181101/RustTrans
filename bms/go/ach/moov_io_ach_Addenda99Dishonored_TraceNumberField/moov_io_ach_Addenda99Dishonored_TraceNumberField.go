package test

import (
	"strings"
	"unicode/utf8"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (Addenda99Dishonored *moov_io_ach_Addenda99Dishonored) TraceNumberField() string {
	return Addenda99Dishonored.stringField(Addenda99Dishonored.TraceNumber, 15)
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

type moov_io_ach_Addenda99Dishonored struct {
	// ID is a client defined string used as a reference to this record.

	// TypeCode Addenda types code '99'

	// DishonoredReturnReasonCode is the return code explaining the dishonorment

	// OriginalEntryTraceNumber is the trace number specifieid in the initial entry

	// OriginalReceivingDFIIdentification is the DFI Identification specifieid in the initial entry

	// ReturnTraceNumber is the TraceNumber used when issuing the return

	// ReturnSettlementDate is the date of return issuing

	// ReturnReasonCode is the initial return code

	// AddendaInformation is additional data

	// TraceNumber is the trace number for dishonorment
	TraceNumber string `json:"traceNumber"`

	// validator is composed for data validation

	// converters is composed for ACH to GoLang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
