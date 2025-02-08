
use std::error::Error;
use std::fmt;
use std::io;
use std::str::FromStr;

const MOOV_IO_ACH_GLPRENOTEDEBIT: i32 = 48;

fn moov_io_ach_check_routing_number(routing_number: &str) -> Result<(), Box<dyn Error>> {
    if routing_number.is_empty() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no routing number provided"
        )));
    }

    let n = routing_number.len();
    if n != 9 {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid routing number length of {}", n)
        )));
    }

    let check = moov_io_ach_calculate_check_digit(routing_number);
    let last = routing_number.chars().last().unwrap() as i32;

    if check != last {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("routing number checksum mismatch: expected {} but got {}", check, last)
        )));
    }

    Ok(())
}

fn moov_io_ach_calculate_check_digit(routing_number: &str) -> i32 {
    0
}

fn moov_io_ach_round_up_10(n: i32) -> i32 {
    n
}

