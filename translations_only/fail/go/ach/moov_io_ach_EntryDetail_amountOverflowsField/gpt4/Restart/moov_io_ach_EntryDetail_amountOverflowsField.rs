
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: Mutex<HashMap<i32, String>> = Mutex::new(moov_io_ach_populate_map(94, "0"));
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchEntryDetail {
    amount: i32,
}

impl MoovIoAchEntryDetail {
    fn amount_overflows_field(&self) -> Result<(), Box<dyn std::error::Error>> {
        let intstr = self.amount.to_string();
        let strstr = self.amount_field();
        if intstr == "0" && strstr == "0000000000" {
            return Ok(()); // both are empty values
        }
        if intstr.len() > strstr.len() {
            return Err(Box::new(std::fmt::Error));
        }
        Ok(())
    }

    fn amount_field(&self) -> String {
        self.numeric_field(self.amount, 10)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() > max as usize {
            s[s.len() - max as usize..].to_string()
        } else {
            let m = (max as usize) - s.len();
            let pad = MOOV_IO_ACH_STRING_ZEROS.lock().unwrap().get(&(m as i32)).unwrap().clone();
            format!("{}{}", pad, s)
        }
    }
}

fn main() {
}
