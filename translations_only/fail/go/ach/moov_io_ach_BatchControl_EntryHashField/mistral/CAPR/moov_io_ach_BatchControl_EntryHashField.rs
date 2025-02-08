

use std::fmt;
use std::collections::HashMap;
use std::str;
use std::iter;

const MAX_INT: i32 = 94;
const ZERO: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = get_pad(m as usize);
            return pad.to_string() + &s;
        }
    }
}

fn get_pad(n: usize) -> String {
    iter::repeat(ZERO).take(n).collect::<String>()
}

fn populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out: HashMap<i32, String> = HashMap::new();
    for i in 0..max {
        out.insert(i, get_pad(i as usize));
    }
    return out;
}

#[derive(Default)]
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
    converters: Option<Box<MoovIoAchConverters>>,
}

impl MoovIoAchBatchControl {
    fn entry_hash_field(&self) -> String {
        self.converters.as_ref().unwrap().numeric_field(self.entry_hash, 10)
    }
}

fn main() {
    let mut map = populate_map(MAX_INT, ZERO);
    let mut batch_control = MoovIoAchBatchControl::default();
    batch_control.converters = Some(Box::new(MoovIoAchConverters));
}

impl fmt::Display for MoovIoAchBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "entry_hash: {}", self.entry_hash)?;
        Ok(())
    }
}

struct MoovIoAchConvertersBox(MoovIoAchConverters);

impl Default for MoovIoAchConvertersBox {
    fn default() -> Self {
        MoovIoAchConvertersBox(MoovIoAchConverters)
    }
}

