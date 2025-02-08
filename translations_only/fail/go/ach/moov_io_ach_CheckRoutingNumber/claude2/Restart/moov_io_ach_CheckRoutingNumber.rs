

use std::error::Error;
use std::fmt;

const MOOV_IO_ACH_GLPRENOTEDEBIT: i32 = 48;

fn moov_io_ach_check_routing_number(routing_number: &str) -> Result<(), Box<dyn Error>> {
    if routing_number.is_empty() {
        return Err(Box::new(fmt::Error));
    }
    let n = routing_number.len();
    if n != 9 {
        return Err(Box::new(fmt::Error));
    }

    let check = moov_io_ach_calculate_check_digit(routing_number);
    let last = routing_number.chars().last().unwrap();
    if check != last {
        return Err(Box::new(fmt::Error));
    }
    Ok(())
}

fn moov_io_ach_calculate_check_digit(routing_number: &str) -> char {
    let n = routing_number.len();
    if n != 8 && n != 9 {
        return '-';
    }

    let mut sum = 0;
    for (i, r) in routing_number.chars().enumerate() {
        if i >= 8 {
            break;
        }
        if !r.is_digit(10) {
            return '-';
        }
        let n = r as i32 - '0' as i32;
        match i {
            0 | 3 | 6 => sum += n * 3,
            1 | 4 | 7 => sum += n * 7,
            2 | 5 => sum += n,
            _ => (),
        }
    }

    let rounded = moov_io_ach_round_up_10(sum);
    let diff = rounded - sum;
    let ascii_value = '0' as u32 + diff as u32;
    char::from_u32(ascii_value).unwrap()
}

fn moov_io_ach_round_up_10(n: i32) -> i32 {
    (f64::ceil(n as f64 / 10.0)) as i32 * 10  
}

