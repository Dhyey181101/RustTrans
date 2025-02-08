

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
            let pad = self.moov_io_ach_string_zeros()
                .get(&m)
                .cloned()
                .unwrap_or_else(|| "0".repeat(m));
            pad + s
        }
    }
}

impl MoovIoAchConverters {
    fn moov_io_ach_populate_map(max: usize, zero: char) -> HashMap<usize, String> {
        let mut out = HashMap::new();
        for i in 0..max {
            out.insert(i, "0".repeat(i));
        }
        out
    }

    fn moov_io_ach_string_zeros(&self) -> &HashMap<usize, String> {
        static INIT: once_cell::sync::Lazy<HashMap<usize, String>> = once_cell::sync::Lazy::new(|| {
            MoovIoAchConverters::moov_io_ach_populate_map(94, '0')
        });
        &INIT
    }
}

#[derive(Default)]
struct MoovIoAchAddenda98Refused {
    trace_sequence_number: String,
}

impl MoovIoAchAddenda98Refused {
    fn trace_sequence_number_field(&self) -> String {
        self.string_field(&self.trace_sequence_number, 7)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        MoovIoAchConverters.string_field(s, max)
    }
}

impl fmt::Display for MoovIoAchAddenda98Refused {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ?, TypeCode: 98, RefusedChangeCode: ?, OriginalTrace: {{{}}}, OriginalDFI: ?, CorrectedData: ?, ChangeCode: ?, TraceSequenceNumber: {{{}}}, TraceNumber: ?",
            self.trace_sequence_number, self.trace_sequence_number
        )
    }
}

