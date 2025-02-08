
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Contested {
    pub date_original_entry_returned: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn date_original_entry_returned_field(&self) -> &str {
        &self.date_original_entry_returned
    }
}

impl FromStr for MoovIoAchAddenda99Contested {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut date_original_entry_returned = String::new();
        let mut chars = s.chars();
        for _ in 0..6 {
            date_original_entry_returned.push(chars.next().unwrap_or('0'));
        }

        Ok(MoovIoAchAddenda99Contested {
            date_original_entry_returned,
        })
    }
}

impl Display for MoovIoAchAddenda99Contested {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "date_original_entry_returned: {}",
            self.date_original_entry_returned
        )
    }
}

pub fn string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        s[..max].to_string()
    } else {
        let mut pad = String::new();
        for _ in 0..(max - ln) {
            pad.push('0');
        }
        pad + s
    }
}

pub fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
