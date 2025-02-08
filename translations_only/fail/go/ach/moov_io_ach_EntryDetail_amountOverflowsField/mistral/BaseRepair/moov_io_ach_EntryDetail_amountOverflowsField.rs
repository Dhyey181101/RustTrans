

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
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchEntryDetail {
    fn amount_overflows_field(&self) -> Result<(), String> {
        let intstr = self.amount.to_string();
        let strstr = self.amount_field();
        if intstr == "0" && strstr == String::from_utf8_lossy(ZEROS).to_string() {
            return Ok(());
        }
        if intstr.len() > strstr.len() {
            Err(format!(
                "does not match formatted value {:?}",
                strstr
            ))
        } else {
            Ok(())
        }
    }

    fn amount_field(&self) -> String {
        self.converters.numeric_field(self.amount, 10)
    }
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.amount_field())
    }
}

fn main() {
    let converters = Box::new(MoovIoAchConverters);
    let ed = MoovIoAchEntryDetail {
        amount: 12345,
        converters,
    };
    match ed.amount_overflows_field() {
        Ok(_) => println!("{}", ed),
        Err(e) => eprintln!("{}", e),
    }
}

