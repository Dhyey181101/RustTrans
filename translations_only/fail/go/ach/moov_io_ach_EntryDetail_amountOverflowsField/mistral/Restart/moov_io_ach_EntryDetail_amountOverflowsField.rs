

use std::fmt;
use std::iter;
use std::string::String;

const ZEROS: &[u8] = b"0000000000";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        if s.len() > max {
            s[s.len() - max..].to_string()
        } else {
            let m = max - s.len();
            let pad = get_zeros(m);
            pad + &s
        }
    }
}

fn get_zeros(n: usize) -> String {
    iter::repeat('0')
        .take(n)
        .collect::<String>()
}

struct MoovIoAchEntryDetail {
    amount: i32,
}

impl MoovIoAchEntryDetail {
    fn amount_overflows_field(&self) -> Result<(), String> {
        let intstr = self.amount.to_string();
        let strstr = self.amount_field();
        if intstr == "0" && strstr == String::from_utf8_lossy(ZEROS) {
            return Ok(());
        }
        if intstr.len() > strstr.len() {
            return Err(format!(
                "does not match formatted value {:?}",
                strstr
            ));
        }
        Ok(())
    }

    fn amount_field(&self) -> String {
        MoovIoAchConverters.numeric_field(self.amount, 10)
    }
}

fn main() {
    let mut ed = MoovIoAchEntryDetail { amount: 12345 };
    match ed.amount_overflows_field() {
        Ok(_) => println!("No errors"),
        Err(e) => println!("Error: {}", e),
    }
}

