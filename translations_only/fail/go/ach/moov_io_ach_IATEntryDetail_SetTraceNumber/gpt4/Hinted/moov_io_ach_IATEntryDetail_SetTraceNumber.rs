
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchIATEntryDetail {
    trace_number: String,
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchIATEntryDetail {
    fn new() -> Self {
        Self {
            trace_number: String::new(),
            converters: Box::new(MoovIoAchConverters {}),
        }
    }

    fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        self.trace_number = format!(
            "{}{}",
            self.converters.string_field(odfi_identification, 8),
            self.converters.numeric_field(seq, 7)
        );
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s.chars().take(max).collect()
        } else {
            let m = max - ln;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                "0".repeat(m) + s
            }
        }
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s.chars().skip(l - max).collect()
        } else {
            let m = max - l;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                "0".repeat(m) + &s
            }
        }
    }
}

fn main() {
    // Example usage
    let mut entry_detail = MoovIoAchIATEntryDetail::new();
    entry_detail.set_trace_number("12345678", 9835776);
    println!("Trace Number: {}", entry_detail.trace_number);
}
