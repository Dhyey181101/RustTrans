

extern crate regex;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt;

const GLPrenoteDebit: u8 = 48;

struct MoovIoAchValidator;

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
                Some((c as u8 - '0' as u8, i))
            } else {
                None
            }
        })
        .map(|(n, i)| {
            match i {
                0 | 3 | 6 => (n as i32) * 3,
                1 | 4 | 7 => (n as i32) * 7,
                _ => n as i32,
            }
        })
        .sum();

    round_up_10(sum) - sum
}

fn round_up_10(n: i32) -> i32 {
    (n + 9) / 10 * 10
}

impl MoovIoAchValidator {
    fn new() -> MoovIoAchValidator {
        MoovIoAchValidator
    }

    fn calculate_check_digit(&self, routing_number: &str) -> i32 {
        calculate_check_digit(routing_number)
    }
}

impl fmt::Debug for MoovIoAchValidator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoovIoAchValidator")
    }
}

fn is_valid_routing_number(routing_number: &str) -> bool {
    let re = Regex::new(r"^\d{8}|\d{9}$").unwrap();
    re.is_match(routing_number)
}

fn main() {
    let validator = MoovIoAchValidator::new();
    let routing_number = "12345678";

    if is_valid_routing_number(routing_number) {
        let num_chars: i32 = routing_number.chars().filter(|c| c.is_digit(10)).count() as i32;
        let mut routing_number_digits = vec![];
        for (i, c) in routing_number.chars().take(8).enumerate() {
            if c >= '0' && c <= '9' {
                routing_number_digits.push((c as u8 - '0' as u8, i));
            }
        }

        let sum: i32 = routing_number_digits
            .iter()
            .map(|&(n, i)| {
                match i {
                    0 | 3 | 6 => (n as i32) * 3,
                    1 | 4 | 7 => (n as i32) * 7,
                    _ => n as i32,
                }
            })
            .sum();

        println!(
            "Check digit for {} is {}",
            routing_number,
            round_up_10(sum) - sum
        );
    } else {
        println!("Invalid routing number");
    }
}

