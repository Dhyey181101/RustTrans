

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters {
    moov_io_ach_string_zeros: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let binding = "".to_string();
        let pad = self.moov_io_ach_string_zeros.get(&m).unwrap_or(&binding);
        pad.clone() + s
    }
}

impl Default for MoovIoAchConverters {
    fn default() -> Self {
        let mut moov_io_ach_string_zeros = HashMap::new();
        for i in 0..1000 {
            moov_io_ach_string_zeros.insert(i, "0".repeat(i));
        }

        MoovIoAchConverters {
            moov_io_ach_string_zeros,
        }
    }
}

#[derive(Default)]
struct MoovIoAchAddenda99Contested {
    date_original_entry_returned: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda99Contested {
    fn date_original_entry_returned_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.date_original_entry_returned, 6)
    }
}

fn main() {
    let mut addenda99contested = MoovIoAchAddenda99Contested::default();
    addenda99contested.date_original_entry_returned = "".to_string();

    let result = addenda99contested.date_original_entry_returned_field();
    println!("{}", result);
}

