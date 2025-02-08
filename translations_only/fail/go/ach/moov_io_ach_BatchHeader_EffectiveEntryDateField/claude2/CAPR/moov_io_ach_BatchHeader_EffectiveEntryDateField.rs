
use std::collections::HashMap;
use std::string::String;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_SPACE_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| moov_io_ach_populate_map(94, " ".to_string()));
static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_owned()));

struct MoovIoAchBatchHeader {
    company_entry_description: Option<String>,
    effective_entry_date: Option<String>,
    converters: Box<MoovIoAchConverters>,
    // other fields omitted
}

impl MoovIoAchBatchHeader {
    fn effective_entry_date_field(&self) -> String {
        if self.company_entry_description == Some("AUTOENROLL".to_string()) {
            self.converters.alpha_field("".to_string(), 6)
        } else {
            let date = self.effective_entry_date.as_ref().unwrap().clone();
            self.converters.string_field(date, 6)  
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
            let m = (max - ln) as i32;
            let pad = MOOV_IO_ACH_SPACE_ZEROS.get(&m).cloned().unwrap_or_default();
            format!("{}{}", s, pad)
        }
    }

    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as i32;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).cloned().unwrap_or_else(|| "0".to_string());
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut map = HashMap::with_capacity(max as usize);
    for i in 0..max {
        map.insert(i, zero.clone().repeat(i as usize));
    }
    map
}

