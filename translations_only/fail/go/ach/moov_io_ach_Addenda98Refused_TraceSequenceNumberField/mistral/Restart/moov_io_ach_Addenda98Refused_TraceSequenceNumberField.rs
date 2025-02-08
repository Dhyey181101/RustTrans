
use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct MoovIoAchConverters {
    get_pad_string: Box<dyn Fn(usize) -> String>,
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = (self.get_pad_string)(m);
            pad + s
        }
    }

    fn new() -> MoovIoAchConverters {
        let mut map = HashMap::new();
        for i in 0..=10 {
            map.insert(i, ZERO.repeat(i));
        }
        MoovIoAchConverters {
            get_pad_string: Box::new(move |count: usize| map[&count].clone()),
        }
    }
}

struct MoovIoAchAddenda98Refused {
    trace_sequence_number: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda98Refused {
    fn trace_sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.trace_sequence_number, 7)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        self.moov_io_ach_converters.string_field(s, max)
    }
}

impl fmt::Display for MoovIoAchAddenda98Refused {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TraceSequenceNumber: {}",
            self.trace_sequence_number_field()
        )
    }
}
