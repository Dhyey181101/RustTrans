
use std::char;

const MOOV_IO_ACH_GLPRENOTEDEBIT: i32 = 48;

struct MoovIoAchValidator;

impl MoovIoAchValidator {
    pub fn calculate_check_digit(&self, routing_number: &str) -> i32 {
        moov_io_ach_calculate_check_digit(routing_number)
    }
}

fn moov_io_ach_calculate_check_digit(routing_number: &str) -> i32 {
    let n = routing_number.len();
    if n != 8 && n != 9 {
        return -1;
    }

    let mut sum = 0;
    for (i, c) in routing_number.chars().enumerate() {
        if i >= 8 {
            break;
        }

        if c < '0' || c > '9' {
            return -1;
        }

        let n = (c as i32) - ('0' as i32);

        match i {
            0 | 3 | 6 => sum += n * 3,
            1 | 4 | 7 => sum += n * 7,
            2 | 5 => sum += n,
            _ => (),
        }
    }

    let rounded = moov_io_ach_round_up_10(sum);
    rounded - sum
}

fn moov_io_ach_round_up_10(n: i32) -> i32 {
    ((n as f64 / 10.0).ceil() * 10.0) as i32
}

