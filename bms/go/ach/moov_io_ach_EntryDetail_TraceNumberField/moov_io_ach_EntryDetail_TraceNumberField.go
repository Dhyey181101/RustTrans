package test

import (
	"strings"
	"unicode/utf8"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (ed *moov_io_ach_EntryDetail) TraceNumberField() string {
	return ed.stringField(ed.TraceNumber, 15)
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

type moov_io_ach_EntryDetail struct {
	// ID is a client defined string used as a reference to this record.

	// TransactionCode if the receivers account is checking, savings, general ledger (GL) or loan.

	// RDFIIdentification is the RDFI's routing number without the last digit.
	// Receiving Depository Financial Institution

	// CheckDigit the last digit of the RDFI's routing number

	// DFIAccountNumber is the receiver's bank account number you are crediting/debiting.
	// It important to note that this is an alphanumeric field, so its space padded, no zero padded

	// Amount Number of cents you are debiting/crediting this account

	// IdentificationNumber an internal identification (alphanumeric) that
	// you use to uniquely identify this Entry Detail Record

	// IndividualName The name of the receiver, usually the name on the bank account

	// DiscretionaryData allows ODFIs to include codes, of significance only to them,
	// to enable specialized handling of the entry. There will be no
	// standardized interpretation for the value of this field. It can either
	// be a single two-character code, or two distinct one-character codes,
	// according to the needs of the ODFI and/or Originator involved. This
	// field must be returned intact for any returned entry.
	//
	// WEB and TEL batches use the Discretionary Data Field as the Payment Type Code

	// AddendaRecordIndicator indicates the existence of an Addenda Record.
	// A value of "1" indicates that one ore more addenda records follow,
	// and "0" means no such record is present.

	// TraceNumber is assigned by the ODFI or software vendor and used as part of identification.
	//
	// The format of trace numbers is the first 8 digits of the ODFI's routing number followed by
	// 7 digits chosen by the ODFI or software vendor.
	//
	// Sequentual or random numbers can be chosen. The only requirement of Nacha is unique trace
	// numbers within a batch and file.
	//
	// Trace Numbers are included in each Entry Detail Record, Corporate Entry Detail Record,
	// and addenda Record.
	//
	// In association with the Batch Number, transmission (File Creation) Date,
	// and File ID Modifier, the Trace Number uniquely identifies an entry within a given file.
	//
	// For addenda Records, the Trace Number will be identical to the Trace Number
	// in the associated Entry Detail Record, since the Trace Number is associated
	// with an entry or item rather than a physical record.
	//
	// Use TraceNumberField for a properly formatted string representation.
	TraceNumber string `json:"traceNumber,omitempty"`
	// Addenda02 for use with StandardEntryClassCode MTE, POS, and SHR

	// Addenda05 for use with StandardEntryClassCode: ACK, ATX, CCD, CIE, CTX, DNE, ENR, WEB, PPD, TRX.

	// Addenda98 for user with Notification of Change

	// Addenda98 for user with Refused Notification of Change

	// Addenda99 for use with Returns

	// Addenda99Contested for use with Contested Dishonored Returns

	// Addenda99Dishonored for use with Dishonored Returns

	// Category defines if the entry is a Forward, Return, or NOC

	// validator is composed for data validation

	// converters is composed for ACH to golang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
