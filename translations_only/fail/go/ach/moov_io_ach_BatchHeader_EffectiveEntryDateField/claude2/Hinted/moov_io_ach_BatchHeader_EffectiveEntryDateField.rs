
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_SPACEZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, " "));
static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0"));

struct MoovIoAchBatchHeader {
    company_entry_description: Option<String>,
    effective_entry_date: String,
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchBatchHeader {
    fn effective_entry_date_field(&self) -> String {
        if self.company_entry_description == Some("AUTOENROLL".to_string()) {
            self.converters.alpha_field("".to_string(), 6)
        } else {
            self.converters.string_field(self.effective_entry_date.clone(), 6)
        }
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn alpha_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            MOOV_IO_ACH_SPACEZEROS.get(&m).unwrap().clone() + &s
        }
    }

    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            MOOV_IO_ACH_STRINGZEROS.get(&m).unwrap().clone() + &s
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

