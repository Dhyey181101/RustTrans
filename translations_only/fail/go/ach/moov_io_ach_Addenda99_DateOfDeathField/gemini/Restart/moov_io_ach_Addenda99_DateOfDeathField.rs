
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Addenda99 {
    pub date_of_death: String,
}

impl Addenda99 {
    pub fn date_of_death_field(&self) -> String {
        if self.date_of_death.is_empty() {
            self.alpha_field("", 6)
        } else {
            self.format_simple_date(self.date_of_death.clone())
        }
    }

    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = SPACE_ZEROS.get(&m).unwrap();
            format!("{}{}", s, pad)
        }
    }

    fn format_simple_date(&self, s: String) -> String {
        if s.is_empty() {
            self.string_field(s, 6)
        } else {
            s
        }
    }

    fn string_field(&self, s: String, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = STRING_ZEROS.get(&m).unwrap();
            format!("{}{}", pad, s)
        }
    }
}

lazy_static! {
    static ref SPACE_ZEROS: HashMap<usize, String> = (0..94).map(|i| (i, " ".repeat(i))).collect();
    static ref STRING_ZEROS: HashMap<usize, String> = (0..94).map(|i| (i, "0".repeat(i))).collect();
}

impl fmt::Display for Addenda99 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Addenda99 {{ date_of_death: {} }}", self.date_of_death)
    }
}

