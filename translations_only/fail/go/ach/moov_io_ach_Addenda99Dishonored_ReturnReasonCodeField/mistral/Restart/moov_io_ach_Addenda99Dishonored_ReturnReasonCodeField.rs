
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda9 {
    digits: Vec<char>,
    map: HashMap<char, i32>,
}

impl Addenda9 {
    fn new() -> Addenda9 {
        Addenda9 {
            digits: Vec::new(),
            map: HashMap::new(),
        }
    }

    fn parse(&mut self, s: &str) {
        self.digits.clear();
        self.map.clear();
        for c in s.chars() {
            if c < ZERO || c > '9' {
                continue;
            }
            self.digits.push(c);
            *self.map.entry(c).or_insert(0) += 1;
        }
    }

    fn add(&mut self, other: &Addenda9) -> i32 {
        let mut result = 0;
        let mut carry = 0;
        for (&a, &b) in self.digits.iter().zip(other.digits.iter()).rev() {
            let sum = a as i32 - ZERO as i32 + b as i32 - ZERO as i32 + carry;
            carry = sum / 10;
            result += (sum % 10) as i32;
        }
        if carry > 0 {
            result += carry;
        }
        result
    }

    fn print(&self) {
        for &c in self.digits.iter().rev() {
            for _ in 0..*self.map.get(&c).unwrap_or(&0) {
                print!("{}", c);
            }
        }
        println!();
    }
}

impl fmt::Display for Addenda9 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &c in self.digits.iter().rev() {
            for _ in 0..*self.map.get(&c).unwrap_or(&0) {
                write!(f, "{}", c)?;
            }
        }
        Ok(())
    }
}
