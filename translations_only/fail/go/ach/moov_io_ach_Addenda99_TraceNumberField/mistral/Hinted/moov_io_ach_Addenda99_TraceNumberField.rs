

use std::collections::HashMap;
use std::fmt;

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
            let pad = self.moov_io_ach_string_zeros.get(&m).cloned().unwrap_or_else(|| "0".repeat(m));
            pad + s
        }
    }

    fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
        let mut out = HashMap::new();
        for i in 0..max {
            out.insert(i, zero.repeat(i));
        }
        out
    }
}

struct MoovIoAchAddenda99 {
    trace_number: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda99 {
    fn trace_number_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.trace_number, 15)
    }
}

impl fmt::Display for MoovIoAchAddenda99 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TraceNumber: {},",
            self.trace_number_field()
        )
    }
}

fn main() {
    let moov_io_ach_string_zeros = MoovIoAchConverters::moov_io_ach_populate_map(94, "0");
    let addenda991 = Box::new(MoovIoAchAddenda99 {
        trace_number: "123456789012345".to_string(),
        moov_io_ach_converters: MoovIoAchConverters {
            moov_io_ach_string_zeros,
        },
    });
    println!("{}", addenda991);
}

