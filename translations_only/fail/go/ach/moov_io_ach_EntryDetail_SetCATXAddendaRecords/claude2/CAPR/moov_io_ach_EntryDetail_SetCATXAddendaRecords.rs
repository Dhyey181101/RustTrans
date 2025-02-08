
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));
static MOOV_IO_ACH_SPACEZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, " ".to_string()));

struct MoovIoAchEntryDetail {
    individual_name: String,
}

impl MoovIoAchEntryDetail {
    fn set_catx_addenda_records(&mut self, i: i32) {
        let count = numeric_field(i, 4);
        let current = self.individual_name.clone();
        if current.len() > 4 {
            self.individual_name = format!("{}{}", count, &current[4..]);
        } else {
            self.individual_name = format!("{}{}{}", count, alpha_field(" ".to_string(), 16), "  ");
        }
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len()-max as usize..].to_string()
    } else {
        let m = (max - s.len() as u32) as i32;
        MOOV_IO_ACH_STRINGZEROS.get(&m).cloned().unwrap_or_else(|| "0".repeat(m as usize)) + &s
    }
}

fn alpha_field(s: String, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = (max - ln) as i32;
        MOOV_IO_ACH_SPACEZEROS.get(&m).cloned().unwrap_or_else(|| " ".repeat(m as usize)) + &s
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchConverters;

