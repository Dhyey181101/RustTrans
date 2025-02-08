
use std::error::Error;
use std::fmt;
use std::str::FromStr;

const MOOV_IO_ACH_GLPRENOTE_DEBIT: i32 = 48;

fn moov_io_ach_check_routing_number(routing_number: &str) -> Result<(), Box<dyn Error>> {
    if routing_number.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "no routing number provided",
        )));
    }
    if routing_number.len() != 9 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("invalid routing number length of {}", routing_number.len()),
        )));
    }

    let check = moov_io_ach_calculate_check_digit(routing_number);
    let last = routing_number.chars().last().unwrap();
    if check != last {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "routing number checksum mismatch: expected {} but got {}",
                check, last
            ),
        )));
    }
    Ok(())
}

fn moov_io_ach_calculate_check_digit(routing_number: &str) -> char {
    if routing_number.len() != 8 && routing_number.len() != 9 {
        return '0';
    }

    let mut sum = 0;
    for (i, c) in routing_number.chars().enumerate() {
        // Don't process check digit of routing number
        if i >= 8 {
            break;
        }

        // Reject anything that's not a digit
        if !c.is_digit(10) {
            return '0'; // only digits are allowed
        }

        // Calculate the check digit
        let n = c.to_digit(10).unwrap();

        match i {
            0 | 3 | 6 => sum += (n * 3) as i32,
            1 | 4 | 7 => sum += (n * 7) as i32,
            2 | 5 => sum += n as i32,
            _ => (),
        }
    }

    let check = moov_io_ach_round_up_10(sum) - sum;
    char::from_digit(check as u32, 10).unwrap()
}

fn moov_io_ach_round_up_10(n: i32) -> i32 {
    ((n as f64 / 10.0).ceil() * 10.0) as i32
}
