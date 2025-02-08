
use std::collections::HashMap;
use std::fmt;
use std::str;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchEntryDetail {
    amount: i32,
}

impl MoovIoAchEntryDetail {
    fn amount_overflows_field(&self) -> Result<(), Box<dyn std::error::Error>> {
        let intstr = self.amount.to_string();
        let strstr = self.amount_field();
        if intstr == "0" && strstr == "0000000000" {
            Ok(())
        } else if intstr.len() > strstr.len() {
            Err(format!("does not match formatted value {}", strstr).into())
        } else {
            Ok(())
        }
    }

    fn amount_field(&self) -> String {
        numeric_field(self.amount, 10)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    let l = s.len();

    if l > max as usize {
        s[l - max as usize..].to_string()
    } else {
        let m = (max - l as u32) as usize;
        "0".repeat(m) + &s
    }
}

struct MoovIoAchConverters;

