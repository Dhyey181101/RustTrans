

use std::collections::HashMap;
use std::fmt;

const MAX_TRACE_NUMBER_LENGTH: usize = 15;

struct MoovIoAchEntryDetail {
    trace_number: String,
    converters: Box<Converters>,
}

struct Converters {
    zeros: HashMap<usize, String>,
}

impl MoovIoAchEntryDetail {
    fn trace_number_field(&self) -> String {
        self.converters.string_field(&self.trace_number, MAX_TRACE_NUMBER_LENGTH)
    }
}

impl Converters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = if let Some(pad) = self.zeros.get(&m) {
            pad.clone()
        } else {
            "0".repeat(m)
        };

        pad + s
    }
}

impl MoovIoAchEntryDetail {
    fn new() -> Self {
        let mut zeros = HashMap::new();
        for i in 0..MAX_TRACE_NUMBER_LENGTH {
            zeros.insert(i, "0".repeat(i));
        }

        MoovIoAchEntryDetail {
            trace_number: String::new(),
            converters: Box::new(Converters { zeros }),
        }
    }
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ trace_number: {}, converters: {{ zeros: {:?} }} }}",
            self.trace_number, self.converters.zeros
        )
    }
}

fn main() {
    let mut entry_detail = MoovIoAchEntryDetail::new();
    entry_detail.trace_number = "1610638400".to_string();
    println!("{}", entry_detail.trace_number_field());

    let mut entry_detail = MoovIoAchEntryDetail::new();
    entry_detail.trace_number = "4294080768".to_string();
    println!("{}", entry_detail.trace_number_field());
}

