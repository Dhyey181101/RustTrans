
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda98Refused {
    pub original_dfi: String,
    pub corrected_data: String,
    pub change_code: String,
    pub trace_sequence_number: String,
    pub trace_number: String,
}

impl MoovIoAchAddenda98Refused {
    pub fn original_dfi_field(&self) -> String {
        self.string_field(&self.original_dfi, 8)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

impl FromStr for MoovIoAchAddenda98Refused {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.split(',');
        Ok(MoovIoAchAddenda98Refused {
            original_dfi: fields.next().unwrap().to_string(),
            corrected_data: fields.next().unwrap().to_string(),
            change_code: fields.next().unwrap().to_string(),
            trace_sequence_number: fields.next().unwrap().to_string(),
            trace_number: fields.next().unwrap().to_string(),
        })
    }
}

impl fmt::Display for MoovIoAchAddenda98Refused {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{}",
            self.original_dfi, self.corrected_data, self.change_code,
            self.trace_sequence_number, self.trace_number
        )
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i));
        }
        out
    };
}
