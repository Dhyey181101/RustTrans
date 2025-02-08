
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
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
        Ok(MoovIoAchAddenda99Contested {
            date_original_entry_returned: s.to_string(),
        })
    }
}

impl fmt::Display for MoovIoAchAddenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MoovIoAchAddenda99Contested {{ date_original_entry_returned: {} }}",
            self.date_original_entry_returned
        )
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        let binding = moov_io_ach_string_zeros();
        let pad = binding.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_string_zeros() -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, String::from_utf8(vec![b'0'; i]).unwrap());
    }
    out
}
