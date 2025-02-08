package test

import (
	"math"
	"strconv"
	"unicode/utf8"
)

var ()

const (
	moov_io_ach_GLPrenoteDebit              = 48
	moov_io_ach_GLZeroDollarRemittanceDebit = 49
)

func (iatBatch *moov_io_ach_IATBatch) calculateEntryHash() int {
	hash := 0
	for _, entry := range iatBatch.Entries {
		entryRDFI, _ := strconv.Atoi(moov_io_ach_aba8(entry.RDFIIdentification))
		hash += entryRDFI
	}

	// EntryHash is essentially the sum of all the RDFI routing numbers in the batch. If the sum exceeds 10 digits
	// (because you have lots of Entry Detail Records), lop off the most significant digits of the sum until there
	// are only 10.
	return iatBatch.leastSignificantDigits(hash, 10)
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

func (c *moov_io_ach_converters) leastSignificantDigits(v int, maxDigits uint) int {
	return v % int(math.Pow10(int(maxDigits)))
}

type moov_io_ach_IATBatch struct {
	// ID is a client defined string used as a reference to this record.

	Entries []*moov_io_ach_IATEntryDetail `json:"IATEntryDetails"`

	// category defines if the entry is a Forward, Return, or NOC

	// Converters is composed for ACH to GoLang Converters
	moov_io_ach_converters
}

type moov_io_ach_IATEntryDetail struct {
	// ID is a client defined string used as a reference to this record.

	// TransactionCode if the receivers account is:
	// Credit (deposit) to checking account '22'
	// Prenote for credit to checking account '23'
	// Debit (withdrawal) to checking account '27'
	// Prenote for debit to checking account '28'
	// Credit to savings account '32'
	// Prenote for credit to savings account '33'
	// Debit to savings account '37'
	// Prenote for debit to savings account '38'

	// RDFIIdentification is the RDFI's routing number without the last digit.
	// Receiving Depository Financial Institution
	RDFIIdentification string `json:"RDFIIdentification"`
	// CheckDigit the last digit of the RDFI's routing number

	// AddendaRecords is the number of Addenda Records

	// Amount Number of cents you are debiting/crediting this account

	// DFIAccountNumber is the receiver's bank account number you are crediting/debiting.
	// It important to note that this is an alphanumeric field, so its space padded, no zero padded

	// OFACScreeningIndicator - Leave blank

	// SecondaryOFACScreeningIndicator - Leave blank

	// AddendaRecordIndicator indicates the existence of an Addenda Record.
	// A value of "1" indicates that one or more addenda records follow,
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

	// Addenda10 is mandatory for IAT entries
	//
	// The Addenda10 Record identifies the Receiver of the transaction and the dollar amount of
	// the payment.

	// Addenda11 is mandatory for IAT entries
	//
	// The Addenda11 record identifies key information related to the Originator of
	// the entry.

	// Addenda12 is mandatory for IAT entries
	//
	// The Addenda12 record identifies key information related to the Originator of
	// the entry.

	// Addenda13 is mandatory for IAT entries
	//
	// The Addenda13 contains information related to the financial institution originating the entry.
	// For inbound IAT entries, the Fourth Addenda Record must contain information to identify the
	// foreign financial institution that is providing the funding and payment instruction for
	// the IAT entry.

	// Addenda14 is mandatory for IAT entries
	//
	// The Addenda14 identifies the Receiving financial institution holding the Receiver's account.

	// Addenda15 is mandatory for IAT entries
	//
	// The Addenda15 record identifies key information related to the Receiver.

	// Addenda16 is mandatory for IAt entries
	//
	// Addenda16 record identifies additional key information related to the Receiver.

	// Addenda17 is optional for IAT entries
	//
	// This is an optional Addenda Record used to provide payment-related data. There i a maximum of up to two of these
	// Addenda Records with each IAT entry.

	// Addenda18 is optional for IAT entries
	//
	// This optional addenda record is used to provide information on each Foreign Correspondent Bank involved in the
	// processing of the IAT entry. If no Foreign Correspondent Bank is involved,the record should not be included.
	// A maximum of five Addenda18 records may be included with each IAT entry.

	// Addenda98 for user with NOC

	// Addenda99 for use with Returns

	// Category defines if the entry is a Forward, Return, or NOC

	// validator is composed for data validation

	// converters is composed for ACH to golang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
