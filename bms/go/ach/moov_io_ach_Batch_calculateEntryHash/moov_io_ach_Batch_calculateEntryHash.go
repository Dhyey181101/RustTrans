package test

import (
	"math"
	"strconv"
	"unicode/utf8"
)

var ()

const (
	moov_io_ach_ADV                         = "moov_io_ach_ADV"
	moov_io_ach_GLPrenoteDebit              = 48
	moov_io_ach_GLZeroDollarRemittanceDebit = 49
)

func (batch *moov_io_ach_Batch) calculateEntryHash() int {
	hash := 0

	if !batch.IsADV() {
		for _, entry := range batch.Entries {
			entryRDFI, _ := strconv.Atoi(moov_io_ach_aba8(entry.RDFIIdentification))
			hash += entryRDFI
		}
	} else {
		for _, entry := range batch.ADVEntries {
			entryRDFI, _ := strconv.Atoi(moov_io_ach_aba8(entry.RDFIIdentification))
			hash += entryRDFI
		}
	}

	// EntryHash is essentially the sum of all the RDFI routing numbers in the batch. If the sum exceeds 10 digits
	// (because you have lots of Entry Detail Records), lop off the most significant digits of the sum until there
	// are only 10.
	return batch.leastSignificantDigits(hash, 10)
}

func (batch *moov_io_ach_Batch) IsADV() bool {
	ok := batch.GetHeader().StandardEntryClassCode == moov_io_ach_ADV
	return ok
}

func (batch *moov_io_ach_Batch) GetHeader() *moov_io_ach_BatchHeader {
	return batch.Header
}

func (c *moov_io_ach_converters) leastSignificantDigits(v int, maxDigits uint) int {
	return v % int(math.Pow10(int(maxDigits)))
}

func moov_io_ach_aba8(rtn string) string {
	n := utf8.RuneCountInString(rtn)
	switch {
	case n > 10:
		return ""
	case n == 10:
		if rtn[0] == '0' || rtn[0] == '1' {
			return rtn[1:9] // ACH server will prefix with space, 0, or 1
		}
		return ""
	case n != 8 && n != 9:
		return ""
	default:
		return rtn[:8]
	}
}

type moov_io_ach_Batch struct {
	// id is a client defined string used as a reference to this record. accessed via ID/SetID

	Header  *moov_io_ach_BatchHeader   `json:"batchHeader"`
	Entries []*moov_io_ach_EntryDetail `json:"entryDetails"`

	ADVEntries []*moov_io_ach_ADVEntryDetail `json:"advEntryDetails,omitempty"`

	// offset holds the information to build an EntryDetail record which
	// balances the batch by debiting or crediting the sum of amounts in the batch.

	// category defines if the entry is a Forward, Return, or NOC

	// Converters is composed for ACH to GoLang Converters
	moov_io_ach_converters
}

type moov_io_ach_EntryDetail struct {
	// ID is a client defined string used as a reference to this record.

	// TransactionCode if the receivers account is checking, savings, general ledger (GL) or loan.

	// RDFIIdentification is the RDFI's routing number without the last digit.
	// Receiving Depository Financial Institution
	RDFIIdentification string `json:"RDFIIdentification"`
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
	RDFIIdentification string `json:"RDFIIdentification"`
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

	// SequenceNumber

	// Addenda99 for use with Returns

	// Category defines if the entry is a Forward, Return, or NOC

	// validator is composed for data validation

	// converters is composed for ACH to golang Converters
	moov_io_ach_converters
}

type moov_io_ach_BatchHeader struct {
	// ID is a client defined string used as a reference to this record.

	// ServiceClassCode ACH Mixed Debits and Credits '200'
	// ACH Credits Only '220'
	// ACH Debits Only '225'

	// CompanyName the company originating the entries in the batch

	// CompanyDiscretionaryData allows Originators and/or ODFIs to include codes (one or more),
	// of significance only to them, to enable specialized handling of all
	// subsequent entries in that batch. There will be no standardized
	// interpretation for the value of the field. This field must be returned
	// intact on any return entry.

	// CompanyIdentification The 9 digit FEIN number (proceeded by a predetermined
	// alpha or numeric character) of the entity in the company name field

	// StandardEntryClassCode
	// Identifies the payment type (product) found within an ACH batch-using a 3-character code.
	// The SEC Code pertains to all items within batch.
	// Determines format of the detail records.
	// Determines addenda records (required or optional PLUS one or up to 9,999 records).
	// Determines rules to follow (return time frames).
	// Some SEC codes require specific data in predetermined fields within the ACH record
	StandardEntryClassCode string `json:"standardEntryClassCode"`

	// CompanyEntryDescription A description of the entries contained in the batch
	//
	//The Originator establishes the value of this field to provide a
	// description of the purpose of the entry to be displayed back to
	// the receive For example, "GAS BILL," "REG. SALARY," "INS. PREM,"
	// "SOC. SEC.," "DTC," "TRADE PAY," "PURCHASE," etc.
	//
	// This field must contain the word "REVERSAL" (left justified) when the
	// batch contains reversing entries.
	//
	// This field must contain the word "RECLAIM" (left justified) when the
	// batch contains reclamation entries.
	//
	// This field must contain the word "NONSETTLED" (left justified) when the
	// batch contains entries which could not settle.

	// CompanyDescriptiveDate currently, the Rules provide that the “Originator establishes this field as the date it
	// would like to see displayed to the Receiver for descriptive purposes.” NACHA recommends that, as desired,
	// the content of this field be formatted using the convention “SDHHMM”, where the “SD” in positions 64- 65 denotes
	// the intent for same-day settlement, and the hours and minutes in positions 66-69 denote the desired settlement
	// time using a 24-hour clock. When electing to use this convention, the ODFI would validate that the field
	// contains either.
	//
	// ODFIs at their discretion may require their Originators to further show intent for
	// same-day settlement using an optional, yet standardized, same-day indicator in the Company Descriptive Date
	// field. The Company Descriptive Date field (5 record, field 8) is an optional field with 6 positions available
	// (positions 64-69).

	// EffectiveEntryDate the date on which the entries are to settle. Format: YYMMDD (Y=Year, M=Month, D=Day)

	// SettlementDate Leave blank, this field is inserted by the ACH operator

	// OriginatorStatusCode refers to the ODFI initiating the Entry.
	// 0 ADV File prepared by an ACH Operator.
	// 1 This code identifies the Originator as a depository financial institution.
	// 2 This code identifies the Originator as a Federal Government entity or agency.

	//ODFIIdentification First 8 digits of the originating DFI transit routing number

	// BatchNumber is assigned in ascending sequence to each batch by the ODFI
	// or its Sending Point in a given file of entries. Since the batch number
	// in the Batch Header Record and the Batch Control Record is the same,
	// the ascending sequence number should be assigned by batch and not by
	// record.

	// validator is composed for data validation

	// converters is composed for ACH to golang Converters
	moov_io_ach_converters
}
