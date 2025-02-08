

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let binding = MoovIoAchStringZeros::new();
        let pad = binding.get(&m);
        if let Some(p) = pad {
            return p.to_string() + s;
        }

        let zero = "0";
        zero.repeat(m) + s
    }
}

struct MoovIoAchAddenda99Contested {
    original_entry_trace_number: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda99Contested {
    fn original_entry_trace_number_field(&self) -> String {
        self.moov_io_ach_converters
            .string_field(&self.original_entry_trace_number, 15)
    }
}

struct MoovIoAchStringZeros {
    data: HashMap<usize, String>,
}

impl MoovIoAchStringZeros {
    fn new() -> MoovIoAchStringZeros {
        let mut data = HashMap::new();
        for i in 0..94 {
            data.insert(i, "0".repeat(i));
        }
        MoovIoAchStringZeros { data }
    }

    fn get(&self, size: &usize) -> Option<&String> {
        self.data.get(size)
    }
}

impl fmt::Display for MoovIoAchAddenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {:?}\nTypeCode: 99\nContestedReturnCode: {:?}\nOriginalEntryTraceNumber: {:?}",
            "", "", self.original_entry_trace_number_field()
        )
    }
}

fn main() {
    let addenda = MoovIoAchAddenda99Contested {
        original_entry_trace_number: String::from("123456789012345"),
        moov_io_ach_converters: MoovIoAchConverters,
    };

    println!("{}", addenda);
}

