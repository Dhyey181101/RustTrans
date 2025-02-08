
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
    static ref MOOV_IO_ACH_SPACE_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, " ");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
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
        let current_len = current.chars().count();
        if current_len > 4 {
            let (_, rest) = current.split_at(4);
            self.individual_name = count + rest;
        } else {
            self.individual_name = count + &self.moov_io_ach_converters.alpha_field(" ", 16) + "  ";
        }
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        if s.len() > max {
            s[s.len() - max..].to_string()
        } else {
            let m = max - s.len();
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                pad.clone() + &s
            } else {
                "0".repeat(m) + &s
            }
        }
    }

    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s.chars().take(max).collect()
        } else {
            let m = max - ln;
            if let Some(pad) = MOOV_IO_ACH_SPACE_ZEROS.get(&m) {
                s.to_string() + pad
            } else {
                s.to_string() + &" ".repeat(max - ln)
            }
        }
    }
}

fn main() {
    let mut entry_detail = MoovIoAchEntryDetail {
        individual_name: "".to_string(),
        moov_io_ach_converters: Box::new(MoovIoAchConverters),
    };
    entry_detail.set_catx_addenda_records(37389568);
    println!("{}", entry_detail.individual_name);
}
