

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &[u8] = b"000000000";

struct MoovIoAchBatchHeader {
    batch_number: i32,
    converters: Box<Converters>,
}

struct Converters {
    field_converters: HashMap<String, Box<dyn Fn(i32) -> String>>,
}

impl MoovIoAchBatchHeader {
    fn batch_number_field(&self) -> String {
        (self.converters.field_converters.get("batch_number").unwrap())(self.batch_number)
    }
}

impl Converters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let len = s.len() as u32;
        if len > max {
            let mut res = String::new();
            for c in s[len as usize - max as usize..].chars() {
                res.push(c);
            }
            res
        } else {
            let m = max - len;
            let pad = get_zeros(m as usize);
            format!("{}{}", pad, s)
        }
    }
}

fn get_zeros(n: usize) -> String {
    if n > 0 {
        let zeros = &ZEROS[..n];
        String::from_utf8_lossy(zeros).to_string()
    } else {
        String::new()
    }
}

impl fmt::Display for MoovIoAchBatchHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ? ServiceClassCode: ? CompanyName: ? CompanyDiscretionaryData: ? CompanyIdentification: ? \
             StandardEntryClassCode: ? CompanyEntryDescription: ? CompanyDescriptiveDate: ? EffectiveEntryDate: ? \
             OriginatorStatusCode: ? ODFIIdentification: ? BatchNumber: {}"
            , self.batch_number_field()
        )
    }
}

