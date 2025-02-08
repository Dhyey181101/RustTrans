

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = MoovIoAchStringZeros::get(&MoovIoAchStringZeros::new(m), &m);
        if pad.is_some() {
            return format!("{}{}", pad.unwrap(), s);
        }

        "0".repeat(m) + s
    }
}

struct MoovIoAchStringZeros {
    data: HashMap<usize, String>,
}

impl MoovIoAchStringZeros {
    fn new(max: usize) -> Self {
        let mut data = HashMap::new();
        for i in 0..=max {
            data.insert(i, ZERO.repeat(i));
        }
        MoovIoAchStringZeros { data }
    }

    fn get(self: &MoovIoAchStringZeros, size: &usize) -> Option<String> {
        self.data.get(size).cloned()
    }
}

struct MoovIoAchAddenda98Refused {
    trace_number: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda98Refused {
    fn trace_number_field(&self) -> String {
        self.string_field(&self.trace_number, 15)
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        self.moov_io_ach_converters.string_field(s, max)
    }
}

impl fmt::Display for MoovIoAchAddenda98Refused {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "traceNumber: {}",
            self.trace_number
        )
    }
}

