
use std::fmt;
use std::str::FromStr;

fn moov_io_ach_check_routing_number(routing_number: &str) -> Result<(), String> {
    if routing_number.is_empty() {
        return Err("no routing number provided".to_string());
    }
    if routing_number.len() != 9 {
        return Err(format!(
            "invalid routing number length of {}",
            routing_number.len()
        ));
    }

    let check = moov_io_ach_calculate_check_digit(routing_number);
    let last = routing_number.chars().last().unwrap();
    if check != last {
        return Err(format!(
            "routing number checksum mismatch: expected {} but got {}",
            check, last
        ));
    }
    Ok(())
}

fn moov_io_ach_calculate_check_digit(routing_number: &str) -> char {
    if routing_number.len() != 8 && routing_number.len() != 9 {
        return '0';
    }

    let mut sum = 0;
    for (i, c) in routing_number.chars().enumerate() {
        if i >= 8 {
            break;
        }
        if !c.is_digit(10) {
            return '0';
        }
        let n = c.to_digit(10).unwrap() as i32;

        match i {
            0 | 3 | 6 => sum += n * 3,
            1 | 4 | 7 => sum += n * 7,
            2 | 5 => sum += n,
            _ => (),
        }
    }

    let check = moov_io_ach_round_up_10(sum) - sum;
    char::from_digit(check as u32, 10).unwrap()
}

fn moov_io_ach_round_up_10(n: i32) -> i32 {
    ((n as f64 / 10.0).ceil() * 10.0) as i32
}
