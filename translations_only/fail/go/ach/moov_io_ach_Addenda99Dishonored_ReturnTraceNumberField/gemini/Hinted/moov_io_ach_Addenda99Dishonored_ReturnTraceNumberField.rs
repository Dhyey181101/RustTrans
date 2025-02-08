
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

struct MoovIoAchAddenda99Dishonored {
    return_trace_number: String,
}

impl MoovIoAchAddenda99Dishonored {
    fn return_trace_number_field(&self) -> &str {
        &self.return_trace_number
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let mut pad = String::new();
            for _ in 0..(max - ln) {
                write!(&mut pad, "0").unwrap();
            }
            pad + s
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn main() {
    let mut addenda99_dishonored = MoovIoAchAddenda99Dishonored {
        return_trace_number: "123456789012345".to_string(),
    };
    let converters = MoovIoAchConverters {};
    println!(
        "{}",
        converters.string_field(addenda99_dishonored.return_trace_number_field(), 15)
    );
}
