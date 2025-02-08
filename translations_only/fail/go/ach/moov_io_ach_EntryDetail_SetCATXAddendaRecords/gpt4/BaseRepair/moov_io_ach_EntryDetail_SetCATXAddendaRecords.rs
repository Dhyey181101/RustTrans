
use std::collections::HashMap;
use std::str::FromStr;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;
static mut MOOV_IO_ACH_SPACE_ZEROS: Option<HashMap<i32, String>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchEntryDetail {
    individual_name: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchEntryDetail {
    fn set_catx_addenda_records(&mut self, i: i32) {
        let count = self.moov_io_ach_converters.numeric_field(i, 4);
        let current = &self.individual_name;
        if current.chars().count() > 4 {
            self.individual_name = count + &current[4..];
        } else {
            self.individual_name = count + &self.moov_io_ach_converters.alpha_field(" ", 16) + "  ";
        }
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() > max as usize {
            s[s.len() - max as usize..].to_string()
        } else {
            let m = max as i32 - s.len() as i32;
            let pad = unsafe { MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m).unwrap().clone() };
            pad + &s
        }
    }

    fn alpha_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s.chars().take(max as usize).collect()
        } else {
            let m = max as i32 - ln as i32;
            let pad = unsafe { MOOV_IO_ACH_SPACE_ZEROS.as_ref().unwrap().get(&m).unwrap().clone() };
            s.to_string() + &pad
        }
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        MOOV_IO_ACH_SPACE_ZEROS = Some(moov_io_ach_populate_map(94, " "));
    }
}
