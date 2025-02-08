
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Dishonored {
    return_settlement_date: String,
}

impl MoovIoAchAddenda99Dishonored {
    pub fn return_settlement_date(&self) -> &str {
        &self.return_settlement_date
    }
}

impl FromStr for MoovIoAchAddenda99Dishonored {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let return_settlement_date = parts.next().unwrap();

        Ok(MoovIoAchAddenda99Dishonored {
            return_settlement_date: return_settlement_date.to_string(),
        })
    }
}

#[derive(Debug)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = &MOOV_IO_ACH_STRING_ZEROS[&(m as usize)];

        format!("{}{}", pad, s)
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
