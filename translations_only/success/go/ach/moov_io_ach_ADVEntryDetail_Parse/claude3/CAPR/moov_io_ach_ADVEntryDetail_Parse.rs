
use std::str::FromStr;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_CHECKING_DEBIT: i32 = 27;
const MOOV_IO_ACH_SAVINGS_ZERO_DOLLAR_REMITTANCE_DEBIT: i32 = 39;
const MOOV_IO_ACH_GLPRENOTE_DEBIT: i32 = 48;
const MOOV_IO_ACH_LOAN_PRENOTE_CREDIT: i32 = 53;
const MOOV_IO_ACH_LOAN_ZERO_DOLLAR_REMITTANCE_CREDIT: i32 = 54;
const MOOV_IO_ACH_CREDIT_SUMMARY: i32 = 87;

impl MoovIoAchAdvEntryDetail {
    fn parse(&mut self, record: &str) {
        if record.chars().count() != MOOV_IO_ACH_RECORD_LENGTH {
            return;
        }

        let runes: Vec<char> = record.chars().collect();

        // 1-1 Always "6"
        // 2-3 is checking credit 22 debit 27 savings credit 32 debit 37
        self.transaction_code = parse_num_field(&record[1..3]);
        // 4-11 the RDFI's routing number without the last digit.
        self.rdfi_identification = parse_string_field(&record[3..11]);
        // 12-12 The last digit of the RDFI's routing number
        self.check_digit = parse_string_field(&record[11..12]);
        // 13-27 The receiver's bank account number you are crediting/debiting
        self.dfi_account_number = Box::new(record[12..27].to_string());
        // 28-39 Number of cents you are debiting/crediting this account
        self.amount = parse_num_field(&record[27..39]);
        // 40-48 Advice Routing Number
        self.advice_routing_number = parse_string_field(&record[39..48]);
        // 49-53 File Identification
        self.file_identification = parse_string_field(&record[48..53]);
        // 54-54 ACH Operator Data
        self.ach_operator_data = parse_string_field(&record[53..54]);
        // 55-76 Individual Name
        self.individual_name = Box::new(record[54..76].to_string());
        // 77-78 allows ODFIs to include codes of significance only to them, normally blank
        self.discretionary_data = Box::new(record[76..78].to_string());
        // 79-79 1 if addenda exists 0 if it does not
        self.addenda_record_indicator = parse_num_field(&record[78..79]);
        // 80-87
        self.ach_operator_routing_number = parse_string_field(&record[79..87]);
        // 88-90
        self.julian_day = parse_num_field(&record[87..90]);
        // 91-94
        self.sequence_number = parse_num_field(&record[90..94]);
    }
}

fn parse_num_field(r: &str) -> i32 {
    i32::from_str(&r.trim()).unwrap_or(0)
}

fn parse_string_field(r: &str) -> String {
    r.trim().to_string()
}

#[derive(Default)]
struct MoovIoAchAdvEntryDetail {
    // TransactionCode representing Accounting Entries
    // Credit for ACH debits originated - 81
    // Debit for ACH credits originated - 82
    // Credit for ACH credits received 83
    // Debit for ACH debits received 84
    // Credit for ACH credits in rejected batches 85
    // Debit for ACH debits in rejected batches - 86
    // Summary credit for respondent ACH activity - 87
    // Summary debit for respondent ACH activity - 88
    transaction_code: i32,
    // RDFIIdentification is the RDFI's routing number without the last digit.
    // Receiving Depository Financial Institution
    rdfi_identification: String,
    // CheckDigit the last digit of the RDFI's routing number
    check_digit: String,
    // DFIAccountNumber is the receiver's bank account number you are crediting/debiting.
    // It important to note that this is an alphanumeric field, so its space padded, no zero padded
    dfi_account_number: Box<String>,
    // Amount Number of cents you are debiting/crediting this account
    amount: i32,
    // AdviceRoutingNumber
    advice_routing_number: String,
    // FileIdentification
    file_identification: String,
    // ACHOperatorData
    ach_operator_data: String,
    // IndividualName The name of the receiver, usually the name on the bank account
    individual_name: Box<String>,
    // DiscretionaryData allows ODFIs to include codes, of significance only to them,
    // to enable specialized handling of the entry. There will be no
    // standardized interpretation for the value of this field. It can either
    // be a single two-character code, or two distinct one-character codes,
    // according to the needs of the ODFI and/or Originator involved. This
    // field must be returned intact for any returned entry.
    discretionary_data: Box<String>,
    // AddendaRecordIndicator indicates the existence of an Addenda Record.
    // A value of "1" indicates that one ore more addenda records follow,
    // and "0" means no such record is present.
    addenda_record_indicator: i32,
    // ACHOperatorRoutingNumber
    ach_operator_routing_number: String,
    // JulianDay
    julian_day: i32,
    // SequenceNumber
    sequence_number: i32,
}
