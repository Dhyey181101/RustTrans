
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
        let pad = get_pad_string(m);
        pad + s
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    let v = out.get(&n).unwrap().to_string();
    v
}

struct MoovIoAchAddenda02 {
    trace_number: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda02 {
    fn trace_number_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.trace_number, 15)
    }
}

impl fmt::Display for MoovIoAchAddenda02 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ?, TypeCode: 02, ReferenceInformationOne: ?, ReferenceInformationTwo: ?, \
             TerminalIdentificationCode: ?, TransactionSerialNumber: ?, TransactionDate: ?, \
             AuthorizationCodeOrExpireDate: ?, TerminalLocation: ?, TerminalCity: ?, \
             TerminalState: ?, TraceNumber: {}",
            self.trace_number_field()
        )
    }
}
