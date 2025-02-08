

use std::fmt;
use std::iter;
use std::string::String;

const ZEROS: &[u8] = b"0000000000";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let len_s = s.len();
        if len_s > max {
            return s[len_s - max..].to_string();
        }
        let padding = iter::repeat('0').take(max - len_s);
        padding.chain(s.chars()).collect()
    }
}

struct MoovIoAchEntryDetail {
    amount: i32,
}

impl MoovIoAchEntryDetail {
    fn new(amount: i32) -> Self {
        MoovIoAchEntryDetail { amount }
    }

    fn numeric_field(&self) -> String {
        let mc = MoovIoAchConverters;
        mc.numeric_field(self.amount, 10)
    }

    fn amount_overflows_field(&self) -> Result<(), String> {
        let intstr = self.amount.to_string();
        let strstr = self.amount_field();
        if intstr == "0" && String::from_utf8_lossy(&ZEROS).to_string() == String::from_utf8_lossy(ZEROS).to_string() {
            return Ok(());
        }
        if intstr.len() > strstr.len() {
            return Err(format!(
                "does not match formatted value {}",
                strstr
            ));
        }
        Ok(())
    }

    fn amount_field(&self) -> String {
        self.numeric_field()
    }
}

fn populate_map(max: i32, zero: &str) -> Box<dyn Iterator<Item = (i32, String)> + 'static> {
    let mut out = vec![];
    for i in 0..max {
        out.push((i, iter::repeat(zero).take(i as usize).collect::<String>()));
    }
    Box::new(out.into_iter())
}

fn main() {
    let _ = populate_map(94, "0");
    let ed = MoovIoAchEntryDetail::new(123);
    match ed.amount_overflows_field() {
        Ok(_) => println!("No errors"),
        Err(e) => println!("Error: {}", e),
    }
}

