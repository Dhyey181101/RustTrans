
use std::collections::HashMap;
use std::str::FromStr;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;
static mut MOOV_IO_ACH_SPACE_ZEROS: Option<HashMap<i32, String>> = None;

struct MoovIoAchEntryDetail {
    individual_name: String,
    converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchEntryDetail {
    fn set_catx_addenda_records(&mut self, i: i32) {
        let count = self.converters.numeric_field(i, 4);
        let current = &self.individual_name;
        if current.chars().count() > 4 {
            self.individual_name = format!("{}{}", count, &current[4..]);
        } else {
            self.individual_name = format!("{}{}  ", count, self.converters.alpha_field(" ", 16));
        }
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() > max as usize {
            s[s.len() - max as usize..].to_string()
        } else {
            let m = max as usize - s.len();
            let pad = unsafe { MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&(m as i32)).unwrap().clone() };
            format!("{}{}", pad, s)
        }
    }

    fn alpha_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s.chars().take(max as usize).collect()
        } else {
            let m = max as usize - ln as usize;
            let pad = unsafe { MOOV_IO_ACH_SPACE_ZEROS.as_ref().unwrap().get(&(m as i32)).unwrap().clone() };
            format!("{}{}", s, pad)
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        MOOV_IO_ACH_SPACE_ZEROS = Some(moov_io_ach_populate_map(94, " "));
    }
}
