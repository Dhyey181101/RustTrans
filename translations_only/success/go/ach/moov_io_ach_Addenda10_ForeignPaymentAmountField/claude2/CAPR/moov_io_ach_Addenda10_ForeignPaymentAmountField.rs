
use std::collections::HashMap;

struct Converters;

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
        let pad = "0".repeat(m as usize);
        pad + &s
    }
}

struct Addenda10 {
    foreign_payment_amount: i32,
}

impl Addenda10 {
    fn foreign_payment_amount_field(&self) -> String {
        numeric_field(self.foreign_payment_amount, 18)
    }
}

