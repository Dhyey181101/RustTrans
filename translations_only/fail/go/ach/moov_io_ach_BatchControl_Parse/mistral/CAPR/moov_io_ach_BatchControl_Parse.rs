

use std::borrow::Borrow;
use std::str;
use std::string::String;
use std::str::FromStr;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_SAVINGS_CREDIT: usize = 32;
const MOOV_IO_ACH_GL_ZERO_DOLLAR_REMITTANCE_CREDIT: usize = 44;
const MOOV_IO_ACH_LOAN_ZERO_DOLLAR_REMITTANCE_CREDIT: usize = 54;
const MOOV_IO_ACH_CREDIT_SUMMARY: usize = 87;

struct MoovIoAchBatchControl {
    // ID is a client defined string used as a reference to this record.

    // ServiceClassCode ACH Mixed Debits and Credits '200'
    // ACH Credits Only '220'
    // ACH Debits Only '225'
    // Constants: MixedCreditsAnDebits (220), CReditsOnly 9220), DebitsOnly (225)
    // Same as 'ServiceClassCode' in BatchHeaderRecord
    service_class_code: i32,
    // EntryAddendaCount is a tally of each Entry Detail Record and each Addenda
    // Record processed, within either the batch or file as appropriate.
    entry_addenda_count: i32,
    // validate the Receiving DFI Identification in each Entry Detail Record is hashed
    // to provide a check against inadvertent alteration of data contents due
    // to hardware failure or program error
    //
    // In this context the Entry Hash is the sum of the corresponding fields in the
    // Entry Detail Records on the file.
    entry_hash: i32,
    // TotalDebitEntryDollarAmount Contains accumulated Entry debit totals within the batch.
    total_debit_entry_dollar_amount: i32,
    // TotalCreditEntryDollarAmount Contains accumulated Entry credit totals within the batch.
    total_credit_entry_dollar_amount: i32,
    // CompanyIdentification is an alphanumeric code used to identify an Originator
    // The Company Identification Field must be included on all
    // prenotification records and on each entry initiated pursuant to such
    // prenotification. The Company ID may begin with the ANSI one-digit
    // Identification Code Designator (ICD), followed by the identification
    // number The ANSI Identification Numbers and related Identification Code
    // Designator (ICD) are:
    //
    // IRS Employer Identification Number (EIN) "1"
    // Data Universal Numbering Systems (DUNS) "3"
    // User Assigned Number "9"
    company_identification: String,
    // MessageAuthenticationCode the MAC is an eight character code derived from a special key used in
    // conjunction with the DES algorithm. The purpose of the MAC is to
    // validate the authenticity of ACH entries. The DES algorithm and key
    // message standards must be in accordance with standards adopted by the
    // American National Standards Institute. The remaining eleven characters
    // of this field are blank.
    message_authentication_code: String,
    // ODFIIdentification the routing number is used to identify the DFI originating entries within a given branch.
    odfi_identification: String,
    // BatchNumber this number is assigned in ascending sequence to each batch by the ODFI
    // or its Sending Point in a given file of entries. Since the batch number
    // in the Batch Header Record and the Batch Control Record is the same,
    // the ascending sequence number should be assigned by batch and not by record.
    batch_number: i32,
    // validator is composed for data validation

    // converters is composed for ACH to golang Converters
}

impl MoovIoAchBatchControl {
    fn parse(&mut self, record: &str, validate_opts: &MoovIoAchValidateOpts) {
        if record.len() != MOOV_IO_ACH_RECORD_LENGTH {
            return;
        }

        self.service_class_code = Self::parse_num_field(&record[0..3]);
        self.entry_addenda_count = Self::parse_num_field(&record[3..9]);
        self.entry_hash = Self::parse_num_field(&record[8..18]);
        self.total_debit_entry_dollar_amount = Self::parse_num_field(&record[17..27]);
        self.total_credit_entry_dollar_amount = Self::parse_num_field(&record[26..36]);
        self.company_identification = Self::parse_string_field_with_opts(&record[35..45], validate_opts);
        self.message_authentication_code = Self::parse_string_field_with_opts(&record[44..73], validate_opts);
        self.odfi_identification = Self::parse_string_field_with_opts(&record[72..87], validate_opts);
        self.batch_number = Self::parse_num_field(&record[86..94]);
    }

    fn parse_num_field(r: &str) -> i32 {
        r.trim().parse::<i32>().unwrap_or(0)
    }

    fn parse_string_field_with_opts(r: &str, opts: &MoovIoAchValidateOpts) -> String {
        if opts.preserve_spaces {
            r.to_string()
        } else {
            Self::parse_string_field(r)
        }
    }

    fn parse_string_field(r: &str) -> String {
        r.trim().to_string()
    }
}

struct MoovIoAchValidateOpts {
    // SkipAll will disable all validation checks of a File. It has no effect when set on records.

    // RequireABAOrigin can be set to enable routing number validation
    // over the ImmediateOrigin file header field.

    // BypassOriginValidation can be set to skip validation for the
    // ImmediateOrigin file header field.
    //
    // This also allows for custom TraceNumbers which aren't prefixed with
    // a routing number as required by the NACHA specification.

    // BypassDestinationValidation can be set to skip validation for the
    // ImmediateDestination file header field.
    //
    // This also allows for custom TraceNumbers which aren't prefixed with
    // a routing number as required by the NACHA specification.

    // CheckTransactionCode allows for custom validation of TransactionCode values
    //
    // Note: Functions cannot be serialized into/from JSON, so this check cannot be used from config files.

    // CustomTraceNumbers disables Nacha specified checks of TraceNumbers:
    // - Ascending order of trace numbers within batches
    // - Trace numbers beginning with their ODFI's routing number
    // - AddendaRecordIndicator is set correctly

    // AllowZeroBatches allows the file to have zero batches

    // AllowMissingFileHeader allows a file to be read without a FileHeader record.

    // AllowMissingFileControl allows a file to be read without a FileControl record.

    // BypassCompanyIdentificationMatch allows batches in which the Company Identification field
    // in the batch header and control do not match.

    // CustomReturnCodes can be set to skip validation for the Return Code field in an Addenda99
    // This allows for non-standard/deprecated return codes (e.g. R97)

    // UnequalServiceClassCode skips equality checks for the ServiceClassCode in each pair of BatchHeader
    // and BatchControl records.

    // AllowUnorderedBatchNumebrs allows a file to be read with unordered batch numbers.

    // AllowInvalidCheckDigit allows the CheckDigit field in EntryDetail to differ from
    // the expected calculation

    // UnequalAddendaCounts skips checking that Addenda Count fields match their expected and computed values.

    // PreserveSpaces keeps the spacing before and after values that normally have spaces trimmed during parsing.
    preserve_spaces: bool,

    // AllowInvalidAmounts will skip verifying the Amount is valid for the TransactionCode and entry type.

}

impl FromStr for MoovIoAchValidateOpts {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MoovIoAchValidateOpts { preserve_spaces: false })
    }
}

