

use std::collections::HashMap;
use std::fmt;
use std::str;

const SERVICE_CLASS_CODE_MIXED: &str = "200";
const SERVICE_CLASS_CODE_CREDITS: &str = "220";
const SERVICE_CLASS_CODE_DEBITS: &str = "225";

struct MoovIoAchBatchControl {
    total_debit_entry_dollar_amount: i32,
    // ... other fields elided ...
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    numeric_field_cache: HashMap<i32, String>,
}

impl Clone for MoovIoAchConverters {
    fn clone(&self) -> Self {
        MoovIoAchConverters {
            numeric_field_cache: self.numeric_field_cache.clone(),
        }
    }
}

impl MoovIoAchBatchControl {
    fn total_debit_entry_dollar_amount_field(&self) -> String {
        let mut converters = self.converters.clone();
        converters.numeric_field(self.total_debit_entry_dollar_amount, 12)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&mut self, n: i32, max: u32) -> String {
        if let Some(cached) = self.numeric_field_cache.get(&n) {
            return cached.clone();
        }

        let s = n.to_string();
        let l = s.len() as u32;
        let result = if l > max {
            String::from(&s[(l - max) as usize..])
        } else {
            let m = max - l;
            let pad = get_zeros(m as usize);
            pad + &s
        };

        self.numeric_field_cache.insert(n, result.clone());
        result
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = String::new();
    for _ in 0..n {
        out.push('0');
    }
    out
}

impl fmt::Display for MoovIoAchBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_debit_entry_dollar_amount: {}" ,
            self.total_debit_entry_dollar_amount
        )
    }
}

fn main() {
    let batch_control = MoovIoAchBatchControl {
        total_debit_entry_dollar_amount: 123,
        converters: Box::new(MoovIoAchConverters {
            numeric_field_cache: HashMap::new(),
        }),
    };
    println!("{}", batch_control.total_debit_entry_dollar_amount_field());
}

