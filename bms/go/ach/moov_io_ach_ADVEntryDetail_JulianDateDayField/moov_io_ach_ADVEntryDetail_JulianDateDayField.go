package test

import (
	"strconv"
	"strings"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (ed *moov_io_ach_ADVEntryDetail) JulianDateDayField() string {
	return ed.numericField(ed.JulianDay, 3)
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

type moov_io_ach_ADVEntryDetail struct {
	// ID is a client defined string used as a reference to this record.

	// TransactionCode representing Accounting Entries
	// Credit for ACH debits originated - 81
	// Debit for ACH credits originated - 82
	// Credit for ACH credits received 83
	// Debit for ACH debits received 84
	// Credit for ACH credits in rejected batches 85
	// Debit for ACH debits in rejected batches - 86
	// Summary credit for respondent ACH activity - 87
	// Summary debit for respondent ACH activity - 88

	// RDFIIdentification is the RDFI's routing number without the last digit.
	// Receiving Depository Financial Institution

	// CheckDigit the last digit of the RDFI's routing number

	// DFIAccountNumber is the receiver's bank account number you are crediting/debiting.
	// It important to note that this is an alphanumeric field, so its space padded, no zero padded

	// Amount Number of cents you are debiting/crediting this account

	// AdviceRoutingNumber

	// FileIdentification

	// ACHOperatorData

	// IndividualName The name of the receiver, usually the name on the bank account

	// DiscretionaryData allows ODFIs to include codes, of significance only to them,
	// to enable specialized handling of the entry. There will be no
	// standardized interpretation for the value of this field. It can either
	// be a single two-character code, or two distinct one-character codes,
	// according to the needs of the ODFI and/or Originator involved. This
	// field must be returned intact for any returned entry.

	// AddendaRecordIndicator indicates the existence of an Addenda Record.
	// A value of "1" indicates that one ore more addenda records follow,
	// and "0" means no such record is present.

	// ACHOperatorRoutingNumber

	// JulianDay
	JulianDay int `json:"julianDay"`
	// SequenceNumber

	// Addenda99 for use with Returns

	// Category defines if the entry is a Forward, Return, or NOC

	// validator is composed for data validation

	// converters is composed for ACH to golang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
