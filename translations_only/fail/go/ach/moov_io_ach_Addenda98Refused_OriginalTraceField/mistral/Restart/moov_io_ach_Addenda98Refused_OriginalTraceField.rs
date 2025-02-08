

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
        let pad = Self::get_pad_string(m);
        pad + s
    }

    fn get_pad_string(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i));
        }
        out[&n].clone()
    }
}

#[derive(Default)]
struct MoovIoAchAddenda98Refused {
    original_trace: String,
}

impl MoovIoAchAddenda98Refused {
    fn original_trace_field(&self) -> String {
        self.string_field(&self.original_trace, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        MoovIoAchConverters.string_field(s, max)
    }
}

impl fmt::Display for MoovIoAchAddenda98Refused {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {:?} TypeCode: {:?} RefusedChangeCode: {:?} OriginalTrace: {:?} OriginalDFI: {:?} CorrectedData: {:?} ChangeCode: {:?} TraceSequenceNumber: {:?} TraceNumber: {:?}",
            "", "98", "", self.original_trace, "", "", "", "", ""
        )
    }
}

