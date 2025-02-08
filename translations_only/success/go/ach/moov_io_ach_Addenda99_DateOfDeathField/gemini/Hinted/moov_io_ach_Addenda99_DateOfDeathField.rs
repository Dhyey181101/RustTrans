
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Addenda99 {
    pub date_of_death: Option<String>,
}

impl Addenda99 {
    pub fn date_of_death_field(&self) -> String {
        match &self.date_of_death {
            Some(date) => date.clone(),
            None => " ".repeat(6),
        }
    }
}

pub struct Converters {}

impl Converters {
    pub fn alpha_field(&self, s: &str, max: usize) -> String {
        let len = s.chars().count();
        if len > max {
            return s[..max].to_string();
        }

        let pad = SPACE_ZEROS.get(&(max - len)).unwrap();
        format!("{}{}", s, pad)
    }

    pub fn format_simple_date(&self, s: &str) -> String {
        if s.is_empty() {
            return self.string_field(s, 6);
        }
        s.to_string()
    }

    pub fn string_field(&self, s: &str, max: usize) -> String {
        let len = s.chars().count();
        if len > max {
            return s[..max].to_string();
        }

        let pad = STRING_ZEROS.get(&(max - len)).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref SPACE_ZEROS: HashMap<usize, String> =
        (0..94).map(|i| (i, " ".repeat(i))).collect();
    static ref STRING_ZEROS: HashMap<usize, String> =
        (0..94).map(|i| (i, "0".repeat(i))).collect();
}
