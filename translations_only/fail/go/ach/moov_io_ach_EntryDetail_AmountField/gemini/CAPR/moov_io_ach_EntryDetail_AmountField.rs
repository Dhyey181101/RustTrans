
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct MoovIoAchEntryDetail {
    pub amount: i32,
}

impl MoovIoAchEntryDetail {
    pub fn amount_field(&self) -> String {
        let binding = "".to_string();
        let pad = moov_io_ach_string_zeros.get(&(10 - self.amount.to_string().len())).unwrap_or(&binding);
        format!("{}{}", pad, self.amount)
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            let mut s = String::new();
            for _ in 0..i {
                s.push('0');
            }
            out.insert(i, s);
        }
        out
    };
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.amount_field())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_field() {
        let ed = MoovIoAchEntryDetail { amount: 12345 };
        assert_eq!(ed.amount_field(), "000012345");
    }
}
