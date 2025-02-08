
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Contested {
    pub original_entry_trace_number: String,
    pub date_original_entry_returned: String,
    pub original_receiving_dfi_identification: String,
    pub original_settlement_date: String,
    pub return_trace_number: String,
    pub return_settlement_date: String,
    pub return_reason_code: String,
    pub dishonored_return_trace_number: String,
    pub dishonored_return_settlement_date: String,
    pub dishonored_return_reason_code: String,
    pub trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn original_entry_trace_number_field(&self) -> String {
        self.string_field(&self.original_entry_trace_number, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap();
            format!("{}{}", pad, s)
        }
    }
}

impl FromStr for MoovIoAchAddenda99Contested {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.split(',');
        Ok(MoovIoAchAddenda99Contested {
            original_entry_trace_number: fields.next().unwrap().to_string(),
            date_original_entry_returned: fields.next().unwrap().to_string(),
            original_receiving_dfi_identification: fields.next().unwrap().to_string(),
            original_settlement_date: fields.next().unwrap().to_string(),
            return_trace_number: fields.next().unwrap().to_string(),
            return_settlement_date: fields.next().unwrap().to_string(),
            return_reason_code: fields.next().unwrap().to_string(),
            dishonored_return_trace_number: fields.next().unwrap().to_string(),
            dishonored_return_settlement_date: fields.next().unwrap().to_string(),
            dishonored_return_reason_code: fields.next().unwrap().to_string(),
            trace_number: fields.next().unwrap().to_string(),
        })
    }
}

impl Display for MoovIoAchAddenda99Contested {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{},{},{},{},{},",
            self.original_entry_trace_number,
            self.date_original_entry_returned,
            self.original_receiving_dfi_identification,
            self.original_settlement_date,
            self.return_trace_number,
            self.return_settlement_date,
            self.return_reason_code,
            self.dishonored_return_trace_number,
            self.dishonored_return_settlement_date,
            self.dishonored_return_reason_code,
            self.trace_number
        )
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from_utf8(vec![b'0'; i]).unwrap());
        }
        out
    };
}
