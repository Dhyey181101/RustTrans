

use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, Box<String>>> = Lazy::new(|| moov_io_ach_populate_map(94, Box::new("0".to_string())));

struct MoovIoAchEntryDetail {
    amount: i32,
}

impl MoovIoAchEntryDetail {
    fn amount_overflows_field(&self) -> Result<(), String> {
        let intstr = self.amount.to_string();
        let strstr = self.amount_field();
        if intstr == "0" && strstr == "0000000000" {
            return Ok(());
        }
        if intstr.len() > strstr.len() {
            return Err(format!("does not match formatted value {}", strstr));
        }
        Ok(())
    }

    fn amount_field(&self) -> String {
        self.numeric_field(self.amount, 10)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = (max as usize) - s.len();
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&Box::new("".to_string())).to_string();
            pad + &s
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: Box<String>) -> HashMap<usize, Box<String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i)));
    }
    out
}

