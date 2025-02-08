
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchEntryDetail {
    pub amount: i32,
}

impl MoovIoAchEntryDetail {
    pub fn amount_overflows_field(&self) -> Result<(), String> {
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

    pub fn amount_field(&self) -> String {
        self.numeric_field(self.amount, 10)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[s.len() - max as usize..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    pub static ref moov_io_ach_string_zeros: HashMap<u32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}

