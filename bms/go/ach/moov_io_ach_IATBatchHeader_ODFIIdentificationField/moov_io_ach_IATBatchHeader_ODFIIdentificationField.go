package test

import (
	"strings"
	"unicode/utf8"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (iatBh *moov_io_ach_IATBatchHeader) ODFIIdentificationField() string {
	return iatBh.stringField(iatBh.ODFIIdentification, 8)
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

type moov_io_ach_IATBatchHeader struct {
	// ID is a client defined string used as a reference to this record.

	// ServiceClassCode ACH Mixed Debits and Credits '200'
	// ACH Credits Only '220'
	// ACH Debits Only '225'

	// IATIndicator - Leave Blank - It is only used for corrected IAT entries

	// ForeignExchangeIndicator is a code indicating currency conversion
	//
	// FV Fixed-to-Variable – Entry is originated in a fixed-value amount
	// and is to be received in a variable amount resulting from the
	// execution of the foreign exchange conversion.
	//
	// VF Variable-to-Fixed – Entry is originated in a variable-value
	// amount based on a specific foreign exchange rate for conversion to a
	// fixed-value amount in which the entry is to be received.
	//
	// FF Fixed-to-Fixed – Entry is originated in a fixed-value amount and
	// is to be received in the same fixed-value amount in the same
	// currency denomination. There is no foreign exchange conversion for
	// entries transmitted using this code. For entries originated in a fixed value
	// amount, the foreign Exchange Reference Field will be space
	// filled.

	// ForeignExchangeReferenceIndicator is a code used to indicate the content of the
	// Foreign Exchange Reference Field and is filled by the gateway operator.
	// Valid entries are:
	// 1 - Foreign Exchange Rate;
	// 2 - Foreign Exchange Reference Number; or
	// 3 - Space Filled

	// ForeignExchangeReference  Contains either the foreign exchange rate used to execute
	// the foreign exchange conversion of a cross-border entry or another reference to the foreign
	// exchange transaction.

	// ISODestinationCountryCode is the two-character code, as approved by the International
	// Organization for Standardization (ISO), to identify the country in which the entry is
	// to be received. Values can be found on the International Organization for Standardization
	// website: www.iso.org.  For entries destined to account holder in the U.S., this would be US.

	// OriginatorIdentification identifies the following:
	// For U.S. entities: the number assigned will be your tax ID
	// For non-U.S. entities: the number assigned will be your DDA number,
	// or the last 9 characters of your account number if it exceeds 9 characters

	// StandardEntryClassCode for consumer and non consumer international payments is IAT
	// Identifies the payment type (product) found within an ACH batch-using a 3-character code.
	// The SEC Code pertains to all items within batch.
	// Determines format of the detail records.
	// Determines addenda records (required or optional PLUS one or up to 9,999 records).
	// Determines rules to follow (return time frames).
	// Some SEC codes require specific data in predetermined fields within the ACH record

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

	// ISOOriginatingCurrencyCode is the three-character code, as approved by the International
	// Organization for Standardization (ISO), to identify the currency denomination in which the
	// entry was first originated. If the source of funds is within the territorial jurisdiction
	// of the U.S., enter 'USD', otherwise refer to International Organization for Standardization
	// website for value: www.iso.org -- (Account Currency)

	// ISODestinationCurrencyCode is the three-character code, as approved by the International
	// Organization for Standardization (ISO), to identify the currency denomination in which the
	// entry will ultimately be settled. If the final destination of funds is within the territorial
	// jurisdiction of the U.S., enter “USD”, otherwise refer to International Organization for
	// Standardization website for value: www.iso.org -- (Payment Currency)

	// EffectiveEntryDate the date on which the entries are to settle. Format: YYMMDD (Y=Year, M=Month, D=Day)

	// SettlementDate Leave blank, this field is inserted by the ACH operator

	// OriginatorStatusCode refers to the ODFI initiating the Entry.
	// 0 ADV File prepared by an ACH Operator.
	// 1 This code identifies the Originator as a depository financial institution.
	// 2 This code identifies the Originator as a Federal Government entity or agency.

	// ODFIIdentification First 8 digits of the originating DFI transit routing number
	// For Inbound IAT Entries, this field contains the routing number of the U.S. Gateway
	// Operator.  For Outbound IAT Entries, this field contains the standard routing number,
	// as assigned by Accuity, that identifies the U.S. ODFI initiating the Entry.
	// Format - TTTTAAAA
	ODFIIdentification string `json:"ODFIIdentification"`

	// BatchNumber is assigned in ascending sequence to each batch by the ODFI
	// or its Sending Point in a given file of entries. Since the batch number
	// in the Batch Header Record and the Batch Control Record is the same,
	// the ascending sequence number should be assigned by batch and not by
	// record.

	// validator is composed for data validation

	// converters is composed for ACH to golang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
