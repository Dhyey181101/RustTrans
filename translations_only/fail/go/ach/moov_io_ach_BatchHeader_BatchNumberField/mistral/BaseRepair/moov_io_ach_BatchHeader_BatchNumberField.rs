

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &[u8] = b"000000000";

struct MoovIoAchBatchHeader {
    batch_number: i32,
    converters: Converters,
}

struct Converters {
    field_converters: HashMap<String, Box<dyn Fn(&MoovIoAchBatchHeader) -> String>>,
}

impl MoovIoAchBatchHeader {
    fn new() -> MoovIoAchBatchHeader {
        MoovIoAchBatchHeader {
            batch_number: 0,
            converters: Converters {
                field_converters: HashMap::new(),
            },
        }
    }

    fn batch_number_field(&self) -> String {
        (self.converters.field_converters.get(&"batch_number".to_string()).unwrap())(self)
    }
}

impl Converters {
    fn numeric_field(&self, header: &MoovIoAchBatchHeader) -> String {
        let s = header.batch_number.to_string();
        let l = s.len() as u32;
        if l > 7 {
            s[(l - 7) as usize..].to_string()
        } else {
            let m = 7 - l;
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

