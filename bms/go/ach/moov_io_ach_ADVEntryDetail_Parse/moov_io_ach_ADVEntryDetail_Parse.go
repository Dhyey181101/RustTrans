package test

import (
	"strconv"
	"strings"
	"unicode/utf8"
)

var ()

const (
	moov_io_ach_RecordLength                     = 94
	moov_io_ach_CheckingDebit                    = 27
	moov_io_ach_SavingsZeroDollarRemittanceDebit = 39
	moov_io_ach_GLPrenoteDebit                   = 48
	moov_io_ach_LoanPrenoteCredit                = 53
	moov_io_ach_LoanZeroDollarRemittanceCredit   = 54
	moov_io_ach_CreditSummary                    = 87
)

func (ed *moov_io_ach_ADVEntryDetail) Parse(record string) {
	if utf8.RuneCountInString(record) != 94 {
		return
	}
	runes := []rune(record)

	// 1-1 Always "6"
	// 2-3 is checking credit 22 debit 27 savings credit 32 debit 37
	ed.TransactionCode = ed.parseNumField(string(runes[1:3]))
	// 4-11 the RDFI's routing number without the last digit.
	ed.RDFIIdentification = ed.parseStringField(string(runes[3:11]))
	// 12-12 The last digit of the RDFI's routing number
	ed.CheckDigit = ed.parseStringField(string(runes[11:12]))
	// 13-27 The receiver's bank account number you are crediting/debiting
	ed.DFIAccountNumber = string(runes[12:27])
	// 28-39 Number of cents you are debiting/crediting this account
	ed.Amount = ed.parseNumField(string(runes[27:39]))
	// 40-48 Advice Routing Number
	ed.AdviceRoutingNumber = ed.parseStringField(string(runes[39:48]))
	// 49-53 File Identification
	ed.FileIdentification = ed.parseStringField(string(runes[48:53]))
	// 54-54 ACH Operator Data
	ed.ACHOperatorData = ed.parseStringField(string(runes[53:54]))
	// 55-76 Individual Name
	ed.IndividualName = string(runes[54:76])
	// 77-78 allows ODFIs to include codes of significance only to them, normally blank
	ed.DiscretionaryData = string(runes[76:78])
	// 79-79 1 if addenda exists 0 if it does not
	ed.AddendaRecordIndicator = ed.parseNumField(string(runes[78:79]))
	// 80-87
	ed.ACHOperatorRoutingNumber = ed.parseStringField(string(runes[79:87]))
	// 88-90
	ed.JulianDay = ed.parseNumField(string(runes[87:90]))
	// 91-94
	ed.SequenceNumber = ed.parseNumField(string(runes[90:94]))
}

func (c *moov_io_ach_converters) parseNumField(r string) (s int) {
	s, _ = strconv.Atoi(strings.TrimSpace(r))
	return s
}

func (c *moov_io_ach_converters) parseStringField(r string) (s string) {
	s = strings.TrimSpace(r)
	return s
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
	TransactionCode int `json:"transactionCode"`
	// RDFIIdentification is the RDFI's routing number without the last digit.
	// Receiving Depository Financial Institution
	RDFIIdentification string `json:"RDFIIdentification"`
	// CheckDigit the last digit of the RDFI's routing number
	CheckDigit string `json:"checkDigit"`
	// DFIAccountNumber is the receiver's bank account number you are crediting/debiting.
	// It important to note that this is an alphanumeric field, so its space padded, no zero padded
	DFIAccountNumber string `json:"DFIAccountNumber"`
	// Amount Number of cents you are debiting/crediting this account
	Amount int `json:"amount"`
	// AdviceRoutingNumber
	AdviceRoutingNumber string `json:"adviceRoutingNumber"`
	// FileIdentification
	FileIdentification string `json:"fileIdentification,omitempty"`
	// ACHOperatorData
	ACHOperatorData string `json:"achOperatorData,omitempty"`
	// IndividualName The name of the receiver, usually the name on the bank account
	IndividualName string `json:"individualName"`
	// DiscretionaryData allows ODFIs to include codes, of significance only to them,
	// to enable specialized handling of the entry. There will be no
	// standardized interpretation for the value of this field. It can either
	// be a single two-character code, or two distinct one-character codes,
	// according to the needs of the ODFI and/or Originator involved. This
	// field must be returned intact for any returned entry.
	DiscretionaryData string `json:"discretionaryData,omitempty"`
	// AddendaRecordIndicator indicates the existence of an Addenda Record.
	// A value of "1" indicates that one ore more addenda records follow,
	// and "0" means no such record is present.
	AddendaRecordIndicator int `json:"addendaRecordIndicator,omitempty"`
	// ACHOperatorRoutingNumber
	ACHOperatorRoutingNumber string `json:"achOperatorRoutingNumber"`
	// JulianDay
	JulianDay int `json:"julianDay"`
	// SequenceNumber
	SequenceNumber int `json:"sequenceNumber"`
	// Addenda99 for use with Returns

	// Category defines if the entry is a Forward, Return, or NOC

	// validator is composed for data validation

	// converters is composed for ACH to golang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
