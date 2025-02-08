
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
        let mut pad = "".to_string();
        for _ in 0..count {
            pad.push('0');
        }
        pad
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

fn main() {
    let mut map = HashMap::new();
    for i in 0..94 {
        let zero = "0".repeat(i);
        map.insert(i, zero);
    }

    let converters = MoovIoAchConverters;
    let addenda = MoovIoAchAddenda99Dishonored {
        original_entry_trace_number: "123456789012345".to_string(),
        // ... other fields ...
        moov_io_ach_converters: converters,
    };

    println!("Original Entry Trace Number: {}", addenda.original_entry_trace_number_field());
}

// Implement Display trait for pretty printing
impl fmt::Display for MoovIoAchAddenda99Dishonored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MoovIoAchAddenda99Dishonored(\n  original_entry_trace_number: {:?},\n  ... other fields ...\n)",
            self.original_entry_trace_number
        )
    }
}
