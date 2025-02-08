

use std::fmt;
use std::iter;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADDENDA_POS: &str = "7";

fn moov_io_ach_populate_map(max: usize, zero: char) -> std::collections::HashMap<usize, String> {
    let mut out: std::collections::HashMap<usize, String> = std::collections::HashMap::new();
    for i in 0..max {
        out.insert(i, iter::repeat(zero).take(i).collect::<String>());
    }
    out
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln: usize = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = MoovIoAchAddenda05::get_pad_string(m);
            format!("{}{}", s, pad)
        }
    }

    fn numeric_field(&self, n: i64, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s[l - max as usize..].to_string()
        } else {
            let m = max - l;
            let pad = MoovIoAchAddenda05::get_pad_string(m as usize);
            format!("{}{}", pad, s)
        }
    }
}

struct MoovIoAchAddenda05 {
    // ID is a client defined string used as a reference to this record.

    // TypeCode Addenda05 types code '05'
    type_code: String,
    // PaymentRelatedInformation
    payment_related_information: String,
    // SequenceNumber is consecutively assigned to each Addenda05 Record following
    // an Entry Detail Record. The first addenda0
    sequence_number: String,
}

impl MoovIoAchAddenda05 {
    fn get_pad_string(n: usize) -> String {
        iter::repeat(' ').take(n).collect::<String>()
    }
}

