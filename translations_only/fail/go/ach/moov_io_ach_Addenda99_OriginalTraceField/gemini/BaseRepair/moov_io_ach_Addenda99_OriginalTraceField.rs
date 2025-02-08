
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct Addenda99 {
    pub original_trace: String,
    pub date_of_death: Option<String>,
    pub original_dfi: Option<String>,
    pub addenda_information: Option<String>,
    pub trace_number: Option<String>,
}

impl Addenda99 {
    pub fn original_trace_field(&self) -> String {
        self.string_field(&self.original_trace, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let binding = "0".repeat(m);
        let pad = STRING_ZEROS.get(&m).unwrap_or(&binding);
        format!("{}{}", pad, s)
    }
}

impl ToString for Addenda99 {
    fn to_string(&self) -> String {
        format!(
            "99{}{}{}{}{}{}",
            self.original_trace_field(),
            self.date_of_death
                .as_ref()
                .map(|d| format!("{} ", d))
                .unwrap_or_default(),
            self.original_dfi
                .as_ref()
                .map(|d| format!("{} ", d))
                .unwrap_or_default(),
            self.addenda_information
                .as_ref()
                .map(|d| format!("{} ", d))
                .unwrap_or_default(),
            self.trace_number
                .as_ref()
                .map(|d| format!("{} ", d))
                .unwrap_or_default(),
            " "
        )
    }
}

impl FromStr for Addenda99 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        Ok(Addenda99 {
            original_trace: parts.next().unwrap().to_string(),
            date_of_death: parts.next().map(ToString::to_string),
            original_dfi: parts.next().map(ToString::to_string),
            addenda_information: parts.next().map(ToString::to_string),
            trace_number: parts.next().map(ToString::to_string),
        })
    }
}

lazy_static! {
    static ref STRING_ZEROS: HashMap<usize, String> =
        (0..100).map(|i| (i, "0".repeat(i))).collect();
}
