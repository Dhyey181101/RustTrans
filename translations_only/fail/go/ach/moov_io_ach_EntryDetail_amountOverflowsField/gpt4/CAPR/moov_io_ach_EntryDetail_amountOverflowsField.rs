
use std::collections::HashMap;
use std::fmt;

struct MoovIoAchEntryDetail {
    amount: i32,
    converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

static MOOV_IO_ACH_STRING_ZEROS: once_cell::sync::Lazy<HashMap<usize, String>> =
    once_cell::sync::Lazy::new(|| moov_io_ach_populate_map(94, "0"));

impl MoovIoAchEntryDetail {
    fn amount_overflows_field(&self) -> Result<(), String> {
        let intstr = self.amount.to_string();
        let strstr = self.amount_field();
        if intstr == "0" && strstr == "0000000000" {
            return Ok(()); // both are empty values
        }
        if intstr.len() > strstr.len() {
            return Err(format!("does not match formatted value {}", strstr));
        }
        Ok(())
    }

    fn amount_field(&self) -> String {
        self.converters.numeric_field(self.amount, 10)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        if s.len() > max {
            s[s.len() - max..].to_string()
        } else {
            let m = max - s.len();
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                // slow path
                format!("{:0>width$}", s, width = max)
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

fn main() {}
