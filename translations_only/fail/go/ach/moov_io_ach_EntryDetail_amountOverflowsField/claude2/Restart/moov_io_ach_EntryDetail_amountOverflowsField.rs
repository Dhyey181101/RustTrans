

use std::collections::HashMap;
use std::fmt;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    HashMap::new()
});

struct MoovIoAchEntryDetail {
    amount: i32,
    converters: MoovIoAchConverters,
}

impl MoovIoAchEntryDetail {
    fn amount_overflows_field(&self) -> Result<(), String> {
        let intstr = self.amount.to_string();
        let strstr = self.converters.numeric_field(self.amount, 10);
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
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[s.len() - max as usize..].to_string()
        } else {
            let m = (max - l) as i32;
            match MOOV_IO_ACH_STRINGZEROS.get(&m) {
                Some(pad) => format!("{}{}", pad, s),
                None => format!("{}{}", "0".repeat(m as usize), s),
            }
        }
    }
}

