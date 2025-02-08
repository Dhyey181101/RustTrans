

use std::fmt;
use std::str;

const MOOV_IO_ACH_GLPrenoteDebit: u8 = 48;

fn moov_io_ach_CheckRoutingNumber(routing_number: &str) -> Result<(), String> {
    if routing_number.is_empty() {
        return Err("no routing number provided".to_string());
    }
    let n = routing_number.len();
    if n != 9 {
        return Err(format!("invalid routing number length of {}", n));
    }

    let check = moov_io_ach_CalculateCheckDigit(routing_number).to_string();
    let last = routing_number.chars().last().unwrap();
    if check != last.to_string() {
        return Err(format!(
            "routing number checksum mismatch: expected {} but got {}",
            check, last
        ));
    }
    Ok(())
}

fn moov_io_ach_CalculateCheckDigit(routing_number: &str) -> i32 {
    let n = routing_number.len();
    if n != 8 && n != 9 {
        return -1;
    }

    let mut sum: i32 = 0;
    for (i, c) in routing_number.chars().enumerate() {
        // Don't process check digit of routing number
        if i >= 8 {
            break;
        }

        // Reject anything that's not a digit
        if c < '0' || c > '9' {
            return -1; // only digits are allowed
        }

        // Calculate the check digit
        let n = c as i32 - '0' as i32;

        match i {
            0 | 3 | 6 => sum += n * 3,
            1 | 4 | 7 => sum += n * 7,
            2 | 5 => sum += n,
            _ => (),
        }
    }

    moov_io_ach_round_up10(sum) - sum
}

fn moov_io_ach_round_up10(n: i32) -> i32 {
    let f = (n as f64 / 10.0).ceil();
    (f * 10.0) as i32
}

