
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Add;
use std::str::FromStr;
use std::string::ToString;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Contested {
    pub return_settlement_date: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn return_settlement_date(&self) -> &str {
        &self.return_settlement_date
    }
}

impl FromStr for MoovIoAchAddenda99Contested {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut addenda99_contested = MoovIoAchAddenda99Contested {
            return_settlement_date: String::new(),
        };

        let mut lines = s.lines();
        while let Some(line) = lines.next() {
            let mut fields = line.splitn(2, ':');
            let field_name = fields.next().unwrap();
            let field_value = fields.next().unwrap();

            match field_name {
                "returnSettlementDate" => addenda99_contested.return_settlement_date = field_value.to_string(),
                _ => (),
            }
        }

        Ok(addenda99_contested)
    }
}

impl Display for MoovIoAchAddenda99Contested {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "returnSettlementDate: {}\n",
            self.return_settlement_date
        )
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = "0".repeat(m as usize);
        format!("{}{}", pad, s)
    }
}

pub fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
