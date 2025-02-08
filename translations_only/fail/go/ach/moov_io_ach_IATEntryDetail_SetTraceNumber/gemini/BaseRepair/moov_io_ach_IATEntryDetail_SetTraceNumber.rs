
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct MoovIoAchIATEntryDetail {
    trace_number: String,
    addenda10: String,
    addenda11: String,
    addenda12: String,
    addenda13: String,
    addenda14: String,
    addenda15: String,
    addenda16: String,
    addenda17: Option<String>,
    addenda18: Option<String>,
    addenda98: Option<String>,
    addenda99: Option<String>,
    category: String,
}

impl MoovIoAchIATEntryDetail {
    fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        self.trace_number = format!(
            "{}{}",
            odfi_identification.chars().take(8).collect::<String>(),
            seq.to_string().chars().rev().take(7).collect::<String>()
        );
    }
}

impl fmt::Display for MoovIoAchIATEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MoovIoAchIATEntryDetail {{ trace_number: {}, addenda10: {}, addenda11: {}, addenda12: {}, addenda13: {}, addenda14: {}, addenda15: {}, addenda16: {}, addenda17: {:?}, addenda18: {:?}, addenda98: {:?}, addenda99: {:?}, category: {} }}",
            self.trace_number, self.addenda10, self.addenda11, self.addenda12, self.addenda13, self.addenda14, self.addenda15, self.addenda16, self.addenda17, self.addenda18, self.addenda98, self.addenda99, self.category
        )
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn main() {
    let mut iat_ed = MoovIoAchIATEntryDetail {
        trace_number: String::new(),
        addenda10: String::new(),
        addenda11: String::new(),
        addenda12: String::new(),
        addenda13: String::new(),
        addenda14: String::new(),
        addenda15: String::new(),
        addenda16: String::new(),
        addenda17: None,
        addenda18: None,
        addenda98: None,
        addenda99: None,
        category: String::new(),
    };

    iat_ed.set_trace_number("123456789", 1234567);

    println!("{}", iat_ed);
}
