
use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = self.get_pad_string(m);
            pad + s
        }
    }

    fn get_pad_string(&self, count: usize) -> String {
        let mut pad_string = "0".repeat(count);
        if count > 0 {
            pad_string.remove(0);
        }
        pad_string
    }
}

struct MoovIoAchAddenda99Dishonored {
    original_entry_trace_number: String,
    // ... other fields ...
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda99Dishonored {
    fn original_entry_trace_number_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.original_entry_trace_number, 15)
    }
}

impl fmt::Display for MoovIoAchAddenda99Dishonored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "original_entry_trace_number: {}",
            self.original_entry_trace_number
        )
    }
}

fn main() {
    let addenda = MoovIoAchAddenda99Dishonored {
        original_entry_trace_number: "12345".to_string(),
        // ... other fields ...
        moov_io_ach_converters: MoovIoAchConverters {},
    };

    println!("{}", addenda.original_entry_trace_number_field());
}
