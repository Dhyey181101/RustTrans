package test

import (
	"strings"
	"unicode/utf8"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (Addenda99Contested *moov_io_ach_Addenda99Contested) ReturnSettlementDateField() string {
	return Addenda99Contested.stringField(Addenda99Contested.ReturnSettlementDate, 3)
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

type moov_io_ach_Addenda99Contested struct {
	// ID is a client defined string used as a reference to this record.

	// TypeCode Addenda types code '99'

	// ContestedReturnCode is the return code explaining the contested dishonorment

	// OriginalEntryTraceNumber is the trace number specifieid in the initial entry

	// DateOriginalEntryReturned is the original entry's date

	// OriginalReceivingDFIIdentification is the DFI Identification specifieid in the initial entry

	// OriginalSettlementDate is the initial date of settlement

	// ReturnTraceNumber is the original returns trace number

	// ReturnSettlementDate is the original return's settlement date
	ReturnSettlementDate string `json:"returnSettlementDate"`

	// ReturnReasonCode is the original return's code

	// DishonoredReturnTraceNumber is the dishonorment's trace number

	// DishonoredReturnSettlementDate is the dishonorment's settlement date

	// DishonoredReturnReasonCode is the dishonorment's return code

	// TraceNumber is the trace number for contesting

	// validator is composed for data validation

	// converters is composed for ACH to GoLang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
