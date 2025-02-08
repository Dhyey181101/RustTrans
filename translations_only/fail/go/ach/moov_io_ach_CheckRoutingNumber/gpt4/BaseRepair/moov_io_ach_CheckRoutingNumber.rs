
use std::error::Error;
use std::fmt;

const MOOV_IO_ACH_GLPRENOTEDEBIT: i32 = 48;

fn moov_io_ach_check_routing_number(routing_number: &str) -> Result<(), Box<dyn Error>> {
    if routing_number.is_empty() {
        return Err("no routing number provided".into());
    }
    if routing_number.chars().count() != 9 {
        return Err(format!("invalid routing number length of {}", routing_number.chars().count()).into());
    }

    let check = moov_io_ach_calculate_check_digit(routing_number).to_string();
    let last = routing_number.chars().last().unwrap().to_string();
    if check != last {
        return Err(format!("routing number checksum mismatch: expected {} but got {}", check, last).into());
    }
    Ok(())
}

fn moov_io_ach_calculate_check_digit(routing_number: &str) -> i32 {
    if routing_number.chars().count() != 8 && routing_number.chars().count() != 9 {
        return -1;
    }

    let mut sum = 0;
    for (i, r) in routing_number.chars().enumerate() {
        if i >= 8 {
            break;
        }

        if !r.is_digit(10) {
            return -1; // only digits are allowed
        }

        let n = r.to_digit(10).unwrap() as i32;

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
    ((n as f64 / 10.0).ceil() * 10.0) as i32
}

fn main() {
    println!("{:?}", moov_io_ach_check_routing_number("123456789"));
}
