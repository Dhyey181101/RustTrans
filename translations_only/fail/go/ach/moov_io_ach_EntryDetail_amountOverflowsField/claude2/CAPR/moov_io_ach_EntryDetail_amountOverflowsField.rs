

use std::collections::HashMap;
use std::convert::From;
use std::fmt;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0"));

struct MoovIoAchEntryDetail {
    amount: i32,
}

impl MoovIoAchEntryDetail {
    fn amount_overflows_field(&self) -> Result<(), String> {
        let intstr = self.amount.to_string();
        let strstr = MoovIoAchConverters::numeric_field(&self.amount, 10);
        if intstr == "0" && strstr == "0000000000" {
            return Ok(());
        }
        if intstr.len() > strstr.len() {
            return Err(format!("does not match formatted value {}", strstr));
        }
        Ok(())
    }

}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(n: &i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s[l - max as usize..].to_string()
        } else {
            let m = (max - l as u32) as usize;
            MOOV_IO_ACH_STRINGZEROS.get(&m).cloned().unwrap_or_else(|| {
                "0".repeat(m)
            }) + &s
        }
    }
}

fn populate_map(max: i32, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i as usize, zero.repeat(i as usize));
    }
    out
}

