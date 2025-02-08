
extern crate regex;
use regex::Regex;
use std::cmp::Ordering;
use std::ops::Add;

const GLPrenoteDebit: u8 = 48;

fn calculate_check_digit(routing_number: &str) -> i32 {
    let n = routing_number.len();
    if n != 8 && n != 9 {
        return -1;
    }

    let sum: i32 = routing_number
        .chars()
        .take(8)
        .enumerate()
        .filter_map(|(i, c)| {
            if c >= '0' && c <= '9' {
                let n = (c as u8 - '0' as u8) as i32;
                match i {
                    0 | 3 | 6 => Some(n * 3),
                    1 | 4 | 7 => Some(n * 7),
                    _ => Some(n),
                }
            } else {
                None
            }
        })
        .sum();

    round_up_10(sum) - sum
}

fn round_up_10(n: i32) -> i32 {
    (n + 5) / 10 * 10
}

struct Validator;

impl Validator {
    fn calculate_check_digit(&self, routing_number: &str) -> i32 {
        calculate_check_digit(routing_number)
    }
}

fn main() {
    let re = Regex::new(r"^\d{8}|\d{9}$").unwrap();
    let input = "12345678";
    if re.is_match(input) {
        println!("Check digit for {} is {}", input, Validator.calculate_check_digit(input));
    } else {
        println!("Invalid routing number");
    }
}
