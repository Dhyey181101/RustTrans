

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters {
    moov_io_ach_string_zeros: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = self.moov_io_ach_string_zeros.get(&m).cloned().unwrap_or_default();
        pad + s
    }
}

impl Default for MoovIoAchConverters {
    fn default() -> Self {
        MoovIoAchConverters {
            moov_io_ach_string_zeros: MoovIoAchConverters::moov_io_ach_populate_map(94, "0"),
        }
    }
}

impl MoovIoAchConverters {
    fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
        let mut out = HashMap::new();
        for i in 0..max {
            out.insert(i, "0".repeat(i));
        }
        out
    }
}

#[derive(Default)]
struct MoovIoAchAddenda02 {
    trace_number: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda02 {
    fn trace_number_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.trace_number, 15)
    }
}

impl fmt::Debug for MoovIoAchAddenda02 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MoovIoAchAddenda02")
            .field("trace_number", &self.trace_number)
            .finish()
    }
}

fn main() {
    let addenda02 = MoovIoAchAddenda02::default();
    println!("{:?}", addenda02);
    println!("Trace Number Field: {}", addenda02.trace_number_field());
}

