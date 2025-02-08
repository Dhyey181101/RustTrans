
extern crate regex;

use regex::Regex;
use std::fmt;
use std::str;

const MOOV_IO_ACH_GLPrenoteDebit: u8 = 48;

fn moov_io_ach_check_routing_number(routing_number: &str) -> Result<(), MoovIoAchError> {
    if routing_number.is_empty() {
        return Err(MoovIoAchError::NoRoutingNumberProvided);
    }
    let n = routing_number.len();
    if n != 9 {
        return Err(MoovIoAchError::InvalidRoutingNumberLength(n));
    }

    let check = moov_io_ach_calculate_check_digit(routing_number);
    let last = routing_number.chars().last().unwrap();
    if check != last {
        return Err(MoovIoAchError::RoutingNumberChecksumMismatch {
            expected: check,
            got: last,
        });
    }
    Ok(())
}

fn moov_io_ach_calculate_check_digit(routing_number: &str) -> char {
    let n = routing_number.len();
    if n != 8 && n != 9 {
        return '-';
    }

    let re = Regex::new(r"[^0-9]").unwrap();
    let routing_number = re.replace_all(routing_number, "").to_string();

    let mut sum = 0;
    for (i, c) in routing_number.chars().enumerate() {
        if i == 8 {
            break;
        }

        let n = c.to_digit(10).unwrap();

        match i {
            0 | 3 | 6 => sum += (n * 3) as i32,
            1 | 4 | 7 => sum += (n * 7) as i32,
            2 | 5 => sum += n as i32,
            _ => (),
        }
    }

    moov_io_ach_round_up_10(sum) as u8 as char
}

fn moov_io_ach_round_up_10(n: i32) -> i32 {
    ((n + 5) / 10) * 10
}

#[derive(Debug)]
enum MoovIoAchError {
    NoRoutingNumberProvided,
    InvalidRoutingNumberLength(usize),
    RoutingNumberChecksumMismatch { expected: char, got: char },
}

impl fmt::Display for MoovIoAchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MoovIoAchError::NoRoutingNumberProvided => write!(f, "no routing number provided"),
            MoovIoAchError::InvalidRoutingNumberLength(n) => {
                write!(f, "invalid routing number length of {}", n)
            }
            MoovIoAchError::RoutingNumberChecksumMismatch { expected, got } => {
                write!(
                    f,
                    "routing number checksum mismatch: expected '{}' but got '{}'",
                    expected, got
                )
            }
        }
    }
}
