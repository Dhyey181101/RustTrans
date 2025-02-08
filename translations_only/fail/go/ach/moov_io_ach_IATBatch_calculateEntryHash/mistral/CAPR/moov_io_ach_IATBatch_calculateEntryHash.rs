
use std::cmp::min;
use std::convert::TryInto;
use std::str;
use std::string::FromUtf8Error;

const MOOV_IO_ACH_GLPRENOTE_CREDIT: u8 = 48;
const MOOV_IO_ACH_GLZERO_DOLLAR_REMITTANCE_DEBIT: u8 = 49;

fn calculate_entry_hash(iat_batch: &IATBatch) -> i32 {
    let mut hash = 0;
    for entry in &iat_batch.entries {
        match entry.rdfi_identification.as_str().parse::<i32>() {
            Ok(entry_rdfi) => hash += entry_rdfi,
            Err(_) => hash += 0,
        }
    }

    hash % (10_i32.pow(10))
}

fn moov_io_ach_aba8(rtn: &str) -> String {
    let n = rtn.chars().count();
    match n {
        10 => {
            if rtn.chars().nth(0).unwrap() == '0' || rtn.chars().nth(0).unwrap() == '1' {
                return rtn[1..8].to_string();
            }
        }
        8 | 9 => return rtn[..min(8, n)].to_string(),
        _ => return String::from(""),
    }
    String::from("")
}

fn least_significant_digits(v: i32, max_digits: u32) -> i32 {
    v % (10_i32.pow(max_digits as u32))
}

#[derive(Debug)]
struct IATEntryDetail {
    rdfi_identification: String,
    // ... other fields
}

#[derive(Debug)]
struct IATBatch {
    entries: Vec<Box<IATEntryDetail>>,
    // ... other fields
}

fn main() {
    // ...
}
