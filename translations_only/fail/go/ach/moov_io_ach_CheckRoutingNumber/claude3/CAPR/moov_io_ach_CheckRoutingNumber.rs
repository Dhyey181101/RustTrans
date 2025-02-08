
use std::error::Error;
use std::fmt;

const MOOV_IO_ACH_GLPRENOTEDEBIT: u8 = 48;

fn moov_io_ach_check_routing_number(routing_number: &str) -> Result<(), Box<dyn Error>> {
    if routing_number.is_empty() {
        return Err("no routing number provided".into());
    }

    let count = routing_number.chars().count();
    if count != 9 {
        return Err(format!("invalid routing number length of {}", count).into());
    }

    let check = moov_io_ach_calculate_check_digit(routing_number).to_string();
    let last = routing_number.chars().last().unwrap().to_string();

    if check != last {
        return Err(format!(
            "routing number checksum mismatch: expected {} but got {}",
            check, last
        )
        .into());
    }

    Ok(())
}

fn moov_io_ach_calculate_check_digit(routing_number: &str) -> i32 {
    let count = routing_number.chars().count();
    if count != 8 && count != 9 {
        return -1;
    }

    let mut sum = 0;
    for (i, c) in routing_number.chars().enumerate() {
        if i >= 8 {
            break;
        }

        if !c.is_digit(10) {
            return -1;
        }

        let n = c.to_digit(10).unwrap() as i32;

        sum += match i {
            0 | 3 | 6 => n * 3,
            1 | 4 | 7 => n * 7,
            2 | 5 => n,
            _ => 0,
        };
    }

    moov_io_ach_round_up10(sum) - sum
}

fn moov_io_ach_round_up10(n: i32) -> i32 {
    (n as f64 / 10.0).ceil() as i32 * 10
}
