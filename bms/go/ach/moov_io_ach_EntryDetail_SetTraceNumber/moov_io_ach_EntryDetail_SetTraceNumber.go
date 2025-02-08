package test

import (
	"strconv"
	"strings"
	"unicode/utf8"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (ed *moov_io_ach_EntryDetail) SetTraceNumber(ODFIIdentification string, seq int) {
	traceNumber := ed.stringField(ODFIIdentification, 8) + ed.numericField(seq, 7)
	ed.TraceNumber = traceNumber

	// Populate TraceNumber of addenda records that should match the Entry's trace number
	if ed.Addenda02 != nil {
		ed.Addenda02.TraceNumber = traceNumber
	}
	if ed.Addenda98 != nil {
		ed.Addenda98.TraceNumber = traceNumber
	}
	if ed.Addenda98Refused != nil {
		ed.Addenda98Refused.TraceNumber = traceNumber
	}
	if ed.Addenda99 != nil {
		ed.Addenda99.TraceNumber = traceNumber
	}
	if ed.Addenda99Contested != nil {
		ed.Addenda99Contested.TraceNumber = traceNumber
	}
	if ed.Addenda99Dishonored != nil {
		ed.Addenda99Dishonored.TraceNumber = traceNumber
	}
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
	Addenda02 *moov_io_ach_Addenda02 `json:"addenda02,omitempty"`
	// Addenda05 for use with StandardEntryClassCode: ACK, ATX, CCD, CIE, CTX, DNE, ENR, WEB, PPD, TRX.

	// Addenda98 for user with Notification of Change
	Addenda98 *moov_io_ach_Addenda98 `json:"addenda98,omitempty"`
	// Addenda98 for user with Refused Notification of Change
	Addenda98Refused *moov_io_ach_Addenda98Refused `json:"addenda98Refused,omitempty"`
	// Addenda99 for use with Returns
	Addenda99 *moov_io_ach_Addenda99 `json:"addenda99,omitempty"`
	// Addenda99Contested for use with Contested Dishonored Returns
	Addenda99Contested *moov_io_ach_Addenda99Contested `json:"addenda99Contested,omitempty"`
	// Addenda99Dishonored for use with Dishonored Returns
	Addenda99Dishonored *moov_io_ach_Addenda99Dishonored `json:"addenda99Dishonored,omitempty"`
	// Category defines if the entry is a Forward, Return, or NOC

	// validator is composed for data validation

	// converters is composed for ACH to golang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}

type moov_io_ach_Addenda02 struct {
	// ID is a client defined string used as a reference to this record.

	// TypeCode Addenda02 type code '02'

	// ReferenceInformationOne may be used for additional reference numbers, identification numbers,
	// or codes that the merchant needs to identify the particular transaction or customer.

	// ReferenceInformationTwo  may be used for additional reference numbers, identification numbers,
	// or codes that the merchant needs to identify the particular transaction or customer.

	// TerminalIdentificationCode identifies an Electronic terminal with a unique code that allows
	// a terminal owner and/or switching network to identify the terminal at which an Entry originated.

	// TransactionSerialNumber is assigned by the terminal at the time the transaction is originated.  The
	// number, with the Terminal Identification Code, serves as an audit trail for the transaction and is
	// usually assigned in ascending sequence.

	// TransactionDate expressed MMDD identifies the date on which the transaction occurred.

	// AuthorizationCodeOrExpireDate indicates the code that a card authorization center has
	// furnished to the merchant.

	// Terminal Location identifies the specific location of a terminal (i.e., street names of an
	// intersection, address, etc.) in accordance with the requirements of Regulation E.

	// TerminalCity Identifies the city in which the electronic terminal is located.

	// TerminalState Identifies the state in which the electronic terminal is located

	// TraceNumber Standard Entry Detail Trace Number
	//
	// Use TraceNumberField for a properly formatted string representation.
	TraceNumber string `json:"traceNumber,omitempty"`
	// validator is composed for data validation

	// converters is composed for ACH to GoLang Converters
	moov_io_ach_converters
}

type moov_io_ach_Addenda98 struct {
	// ID is a client defined string used as a reference to this record.

	// TypeCode Addenda types code '98'

	// ChangeCode field contains a standard code used by an ACH Operator or RDFI to describe the reason for a change Entry.
	// Must exist in changeCodeDict

	// OriginalTrace This field contains the Trace Number as originally included on the forward Entry or Prenotification.
	// The RDFI must include the Original Entry Trace Number in the Addenda Record of an Entry being returned to an ODFI,
	// in the Addenda Record of an 98, within an Acknowledgment Entry, or with an RDFI request for a copy of an authorization.

	// OriginalDFI field contains the Receiving DFI Identification (addenda.RDFIIdentification) as originally included on the forward Entry or Prenotification that the RDFI is returning or correcting.

	// CorrectedData is the corrected data

	// TraceNumber matches the Entry Detail Trace Number of the entry being returned.
	//
	// Use TraceNumberField for a properly formatted string representation.
	TraceNumber string `json:"traceNumber,omitempty"`

	// validator is composed for data validation

	// converters is composed for ACH to GoLang Converters
	moov_io_ach_converters
}

type moov_io_ach_Addenda98Refused struct {
	// ID is a client defined string used as a reference to this record.

	// TypeCode Addenda types code '98'

	// RefusedChangeCode is the code specifying why the Notification of Change is being refused.

	// OriginalTrace This field contains the Trace Number as originally included on the forward Entry or Prenotification.
	// The RDFI must include the Original Entry Trace Number in the Addenda Record of an Entry being returned to an ODFI,
	// in the Addenda Record of an 98, within an Acknowledgment Entry, or with an RDFI request for a copy of an authorization.

	// OriginalDFI field contains the Receiving DFI Identification (addenda.RDFIIdentification) as originally included on the
	// forward Entry or Prenotification that the RDFI is returning or correcting.

	// CorrectedData is the corrected data

	// ChangeCode field contains a standard code used by an ACH Operator or RDFI to describe the reason for a change Entry.

	// TraceSequenceNumber is the last seven digits of the TraceNumber in the original Notification of Change

	// TraceNumber matches the Entry Detail Trace Number of the entry being returned.
	//
	// Use TraceNumberField for a properly formatted string representation.
	TraceNumber string `json:"traceNumber"`

	// validator is composed for data validation

	// converters is composed for ACH to GoLang Converters
	moov_io_ach_converters
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

	// TraceNumber matches the Entry Detail Trace Number of the entry being returned.
	//
	// Use TraceNumberField for a properly formatted string representation.
	TraceNumber string `json:"traceNumber,omitempty"`

	// validator is composed for data validation

	// converters is composed for ACH to GoLang Converters
	moov_io_ach_converters
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

	// ReturnReasonCode is the original return's code

	// DishonoredReturnTraceNumber is the dishonorment's trace number

	// DishonoredReturnSettlementDate is the dishonorment's settlement date

	// DishonoredReturnReasonCode is the dishonorment's return code

	// TraceNumber is the trace number for contesting
	TraceNumber string `json:"traceNumber"`

	// validator is composed for data validation

	// converters is composed for ACH to GoLang Converters
	moov_io_ach_converters
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
