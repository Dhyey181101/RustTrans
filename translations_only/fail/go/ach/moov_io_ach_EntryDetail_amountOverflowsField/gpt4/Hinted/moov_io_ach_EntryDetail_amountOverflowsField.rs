
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;

struct MoovIoAchEntryDetail {
    amount: i32,
}

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
        MoovIoAchConverters::numeric_field(self.amount, 10)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s.chars().skip((l - max) as usize).collect()
        } else {
            let m = (max - l) as i32;
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS
                    .as_ref()
                    .unwrap()
                    .get(&m)
                    .unwrap_or(&String::new())
                    .clone()
            };
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
