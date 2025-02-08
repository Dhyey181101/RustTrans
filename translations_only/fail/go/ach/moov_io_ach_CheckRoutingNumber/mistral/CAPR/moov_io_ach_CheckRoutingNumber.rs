

extern crate regex;

use regex::Regex;
use std::fmt;
use std::str;

const MOOV_IO_ACH_GLPrenoteDebit: u8 = 48;

fn moov_io_ach_check_routing_number(routing_number: &str) -> Result<(), MoovIoAchError> {
    if routing_number.is_empty() {
        return Err(MoovIoAchError::new("NoRoutingNumberProvided"));
    }

    let n = routing_number.len();
    if n != 9 {
        return Err(MoovIoAchError::new(&format!("InvalidRoutingNumberLength({})", n)));
    }

    let check = moov_io_ach_calculate_check_digit(routing_number);
    let last = routing_number.chars().last().unwrap();
    if check != last {
        return Err(MoovIoAchError::new(&format!(
            "RoutingNumberChecksumMismatch {{ expected: {}, got: {} }}",
            check, last
        )));
    }

    Ok(())
}

fn moov_io_ach_calculate_check_digit(routing_number: &str) -> char {
    let n = routing_number.len();
    if n != 8 && n != 9 {
        return '-';
    }

    let mut sum = 0;
    for (i, c) in routing_number.chars().enumerate() {
        if i >= 8 {
            break;
        }

        if c < '0' || c > '9' {
            return '-'; // only digits are allowed
        }

        let n = c as u8 - '0' as u8;

        match i {
            0 | 3 | 6 => sum += (n * 3) as i32,
            1 | 4 | 7 => sum += (n * 7) as i32,
            2 | 5 => sum += n as i32,
            _ => (),
        }
    }

    let check = moov_io_ach_round_up_10(sum) - sum;
    ('0' as u8 + check as u8) as char
}

fn moov_io_ach_round_up_10(n: i32) -> i32 {
    ((n + 5) / 10) * 10
}

#[derive(Debug)]
struct MoovIoAchError {
    message: String,
}

impl fmt::Display for MoovIoAchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoovIoAchError: {}", self.message)
    }
}

impl std::error::Error for MoovIoAchError {}

impl MoovIoAchError {
    fn new(message: &str) -> MoovIoAchError {
        MoovIoAchError {
            message: message.to_string(),
        }
    }
}

impl From<regex::Error> for MoovIoAchError {
    fn from(e: regex::Error) -> Self {
        MoovIoAchError::new(&format!("Invalid regex: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moov_io_ach_check_routing_number() {
        assert_eq!(
            moov_io_ach_check_routing_number("111000025"),
            Ok(())
        );
        assert_eq!(
            moov_io_ach_check_routing_number("11100002X"),
            Err(MoovIoAchError::new("RoutingNumberChecksumMismatch { expected: 5, got: X }"))
        );
        assert_eq!(
            moov_io_ach_check_routing_number(""),
            Err(MoovIoAchError::new("NoRoutingNumberProvided"))
        );
        assert_eq!(
            moov_io_ach_check_routing_number("12345678901"),
            Err(MoovIoAchError::new(&format!(
                "InvalidRoutingNumberLength(10)"
            )))
        );
    }
}

