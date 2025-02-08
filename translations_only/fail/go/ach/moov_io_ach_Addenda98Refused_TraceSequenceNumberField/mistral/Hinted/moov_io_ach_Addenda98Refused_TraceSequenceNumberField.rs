

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters {
    moov_io_ach_string_zeros: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn new() -> MoovIoAchConverters {
        MoovIoAchConverters {
            moov_io_ach_string_zeros: HashMap::from([(94, "0".repeat(94))]),
        }
    }

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

struct MoovIoAchAddenda98Refused {
    trace_sequence_number: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda98Refused {
    fn trace_sequence_number_field(&self) -> String {
        self.string_field(&self.trace_sequence_number, 7)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        self.moov_io_ach_converters.string_field(s, max)
    }
}

impl fmt::Display for MoovIoAchAddenda98Refused {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ?, TypeCode: 98, RefusedChangeCode: ?, OriginalTrace: {}, OriginalDFI: ?, CorrectedData: ?, ChangeCode: ?, TraceSequenceNumber: {}, TraceNumber: {}",
            self.trace_sequence_number,
            self.trace_sequence_number_field(),
            self.trace_sequence_number
        )
    }
}

