

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters {
    pad_strings: HashMap<usize, String>,
}

impl Default for MoovIoAchConverters {
    fn default() -> Self {
        let mut out = HashMap::new();
        for i in 0..=10 {
            out.insert(i, "0".repeat(i));
        }
        MoovIoAchConverters { pad_strings: out }
    }
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            self.pad_strings[&m].clone() + s
        }
    }
}

#[derive(Default)]
struct MoovIoAchAddenda99Dishonored {
    original_receiving_dfi_identification: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda99Dishonored {
    fn original_receiving_dfi_identification_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.original_receiving_dfi_identification, 8)
    }
}

impl fmt::Display for MoovIoAchAddenda99Dishonored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ?, TypeCode: '99', DishonoredReturnReasonCode: ?, OriginalEntryTraceNumber: ?, OriginalReceivingDFIIdentification: {}, ReturnTraceNumber: ?, ReturnSettlementDate: ?, ReturnReasonCode: ?, AddendaInformation: ?, TraceNumber: ?, validator: ?",
            self.original_receiving_dfi_identification_field()
        )
    }
}

