
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
        if current.chars().count() > 4 {
            self.individual_name = count + &current.chars().skip(4).collect::<String>();
        } else {
            self.individual_name = count + &self.moov_io_ach_converters.alpha_field(" ", 16) + "  ";
        }
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s.chars().skip((s.len() as u32 - max) as usize).collect()
        } else {
            let m = (max - s.len() as u32) as usize;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                pad.clone() + &s
            } else {
                // slow path
                "0".repeat(m) + &s
            }
        }
    }

    fn alpha_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s.chars().take(max as usize).collect()
        } else {
            let m = (max - ln) as usize;
            if let Some(pad) = MOOV_IO_ACH_SPACE_ZEROS.get(&m) {
                s.to_string() + pad
            } else {
                // slow path
                s.to_string() + &" ".repeat(max as usize - ln as usize)
            }
        }
    }
}

fn main() {}
