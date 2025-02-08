

use std::fmt;
use std::iter;
use std::string::String;

const ZEROS: [&str; 94] = ["0"; 94];

struct EntryDetail {
    amount: i32,
}

struct Converters;

impl EntryDetail {
    fn amount_overflows_field(&self) -> Result<(), String> {
        let intstr = self.amount.to_string();
        let strstr = self.amount_field();
        if intstr == "0" && strstr == "0000000000" {
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
        Converters::numeric_field(self.amount, 10)
    }
}

impl Converters {
    fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return String::from(&s[(l - max) as usize..]);
        }
        let m = max - l;
        let pad = ZEROS[m as usize].to_string();
        pad + &s
    }
}

fn populate_map(max: i32, zero: &str) -> Vec<(i32, String)> {
    let mut out: Vec<(i32, String)> = Vec::new();
    for i in 0..max {
        let z = iter::repeat(zero).take(i as usize).collect::<String>();
        out.push((i, z));
    }
    out
}

fn main() {
    let mut map = populate_map(94, "0");
    let ed = EntryDetail { amount: 123 };
    match ed.amount_overflows_field() {
        Ok(_) => println!("No errors"),
        Err(e) => println!("Error: {}", e),
    }
}

