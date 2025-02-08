

use std::collections::HashMap;
use std::fmt;

#[derive(Default)]
struct MoovIoAchConverters {
    moov_io_ach_string_zeros: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = self.moov_io_ach_string_zeros.get(&m).cloned().unwrap_or_default();
            format!("{}{}", pad, s)
        }
    }
}

impl MoovIoAchConverters {
    fn moov_io_ach_populate_map(max: usize, zero: char) -> HashMap<usize, String> {
        let mut out = HashMap::new();
        for i in 0..max {
            out.insert(i, ("0".repeat(i)).to_string());
        }
        out
    }

    pub fn new() -> Self {
        let moov_io_ach_string_zeros = Self::moov_io_ach_populate_map(100, '0');
        MoovIoAchConverters { moov_io_ach_string_zeros }
    }
}

#[derive(Default)]
struct MoovIoAchAddenda99Dishonored {
    dishonored_return_reason_code: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda99Dishonored {
    fn dishonored_return_reason_code(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.dishonored_return_reason_code, 3)
    }
}

impl fmt::Display for MoovIoAchAddenda99Dishonored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MoovIoAchAddenda99Dishonored {{\n  dishonored_return_reason_code: {:?},\n}}",
            self.dishonored_return_reason_code
        )
    }
}

fn main() {
    let addenda = MoovIoAchAddenda99Dishonored {
        dishonored_return_reason_code: "1234567890123456789012345678901234567890".to_string(),
        ..Default::default()
    };
    println!("{}", addenda);
}

