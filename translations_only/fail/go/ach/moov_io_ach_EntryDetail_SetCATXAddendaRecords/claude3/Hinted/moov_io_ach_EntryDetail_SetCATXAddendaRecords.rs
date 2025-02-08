
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0"));
static MOOV_IO_ACH_SPACE_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, " "));

impl EntryDetail {
    fn set_catx_addenda_records(&mut self, i: usize) {
        let count = self.converters.numeric_field(i, 4);
        let mut current = self.individual_name.clone();
        if current.chars().count() > 4 {
            self.individual_name = count + &current[4..];
        } else {
            self.individual_name = count + &self.converters.alpha_field(" ".to_string(), 16) + "  ";
        }
    }
}

impl Converters {
    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        if s.len() > max as usize {
            s[(s.len() - max as usize)..].to_string()
        } else {
            let m = (max as usize) - s.len();
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                pad.to_string() + &s
            } else {
                "0".repeat(m) + &s
            }
        }
    }

    fn alpha_field(&self, s: String, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..(max as usize)].to_string()
        } else {
            let m = (max - ln) as usize;
            if let Some(pad) = MOOV_IO_ACH_SPACE_ZEROS.get(&m) {
                s + pad
            } else {
                s + &" ".repeat(m)
            }
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

struct EntryDetail {
    individual_name: String,
    converters: Box<Converters>,
}

struct Converters {}
