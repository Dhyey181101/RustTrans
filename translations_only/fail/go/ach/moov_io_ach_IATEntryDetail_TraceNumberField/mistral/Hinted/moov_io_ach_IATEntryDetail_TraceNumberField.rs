

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = Self::get_pad_string(m);
        pad.chars().rev().chain(s.chars()).collect()
    }

    fn get_pad_string(n: usize) -> String {
        let mut map = HashMap::new();
        for i in 0..n {
            map.insert(i, "0".repeat(i));
        }
        map.get(&n).unwrap().to_string()
    }
}

#[derive(Default)]
struct MoovIoAchIatEntryDetail {
    trace_number: String,
}

impl MoovIoAchIatEntryDetail {
    fn trace_number(&self) -> String {
        MoovIoAchConverters::string_field(&MoovIoAchConverters, &self.trace_number, 15)
    }
}

impl fmt::Display for MoovIoAchIatEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ? \
             TransactionCode: ? \
             RDFIIdentification: ? \
             CheckDigit: ? \
             AddendaRecords: ? \
             Amount: ? \
             DFIAccountNumber: ? \
             OFACScreeningIndicator: ? \
             SecondaryOFACScreeningIndicator: ? \
             AddendaRecordIndicator: ? \
             TraceNumber: {}\
             Addenda10: ? \
             Addenda11: ? \
             Addenda12: ? \
             Addenda13: ? \
             Addenda14: ? \
             Addenda15: ? \
             Addenda16: ? \
             Addenda17: ? \
             Addenda18: ? \
             Addenda98: ? \
             Addenda99: ? \
             Category: ? ",
            self.trace_number()
        )
    }
}

impl fmt::Display for MoovIoAchConverters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoovIoAchConverters")
    }
}

fn main() {
    let mut iat_entry_detail = MoovIoAchIatEntryDetail::default();
    iat_entry_detail.trace_number = "12345678901234".to_string();
    println!("{}", iat_entry_detail);
}

