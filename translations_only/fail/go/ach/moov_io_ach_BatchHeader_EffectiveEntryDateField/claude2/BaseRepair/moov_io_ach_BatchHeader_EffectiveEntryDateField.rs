
use std::collections::HashMap;
use std::string::String;
use once_cell::sync::Lazy;

static moov_io_ach_space_zeros: Lazy<HashMap<i32, String>> = Lazy::new(|| moov_io_ach_populate_map(94, " ".to_string()));
static moov_io_ach_string_zeros: Lazy<HashMap<i32, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchBatchHeader {
    company_entry_description: Option<String>,
    effective_entry_date: Option<String>,
    // other fields omitted
}

impl MoovIoAchBatchHeader {
    fn effective_entry_date_field(&self) -> String {
        if self.company_entry_description == Some("AUTOENROLL".to_string()) {
            self.alpha_field("".to_string(), 6)
        } else {
            self.string_field(self.effective_entry_date.as_ref().unwrap_or(&"".to_string()), 6)
        }
    }
    
    fn alpha_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as i32;
            let pad = moov_io_ach_space_zeros.get(&m).cloned().unwrap_or_default();
            format!("{}{}", s, pad)
        }
    }

    fn string_field(&self, s: &String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as i32;
            let pad = moov_io_ach_string_zeros.get(&m).cloned().unwrap_or_else(|| "0".repeat(m as usize));
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

