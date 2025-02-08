
use std::collections::HashMap;
use std::string::String;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_SPACEZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| moov_io_ach_populate_map(94, " ".to_string()));
static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchBatchHeader {
    company_entry_description: Option<String>,
    effective_entry_date: Option<String>,
    converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn alpha_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as i32;
            let pad = MOOV_IO_ACH_SPACEZEROS.get(&m).cloned().unwrap_or_else(|| " ".repeat(m as usize));
            format!("{}{}", s, pad)
        }
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as i32;
            let pad = MOOV_IO_ACH_STRINGZEROS.get(&m).cloned().unwrap_or_else(|| "0".repeat(m as usize));
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

impl MoovIoAchBatchHeader {
    fn effective_entry_date_field(&self) -> String {
        if self.company_entry_description == Some("AUTOENROLL".to_string()) {
            self.converters.alpha_field("", 6)
        } else {
            self.converters.string_field(
                self.effective_entry_date.as_deref().unwrap_or(&"".to_string()),
                 6,
            )
        }
    }
}

