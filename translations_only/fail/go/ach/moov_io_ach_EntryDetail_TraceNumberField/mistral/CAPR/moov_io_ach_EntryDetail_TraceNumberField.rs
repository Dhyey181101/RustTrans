
use std::collections::HashMap;
use std::fmt;

struct MoovIoAchEntryDetail {
    trace_number: String,
    // ... other fields ...
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters;

impl MoovIoAchEntryDetail {
    fn trace_number_field(&self) -> String {
        self.string_field(self.trace_number.clone(), 15)
    }

    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            let pad = get_zero_string(m);
            format!("{}{}", pad, s)
        }
    }
}

fn get_zero_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].to_string()
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {:<2} TRN: {:<15} RDFI: {:<15} CKD: {:<15} DFI: {:<15} AMT: {:<15} IDN: {:<15} NME: {:<15} DSC: {:<15} AAI: {:<15} TNC: {:<15}",
            "ID", self.trace_number,
            "<RDFI>", "<CKD>", "<DFI>", "<AMT>", "<IDN>", "<NME>", "<DSC>", "<AAI>", "<TNC>",
        )
    }
}

struct MoovIoAchConvertersBox(MoovIoAchConverters);

impl MoovIoAchEntryDetail {
    fn new() -> Self {
        MoovIoAchEntryDetail {
            trace_number: String::new(),
            // ... initialize other fields ...
            converters: Box::new(MoovIoAchConverters),
        }
    }
}

fn main() {
    let ed = MoovIoAchEntryDetail::new();
    println!("{}", ed);
}
