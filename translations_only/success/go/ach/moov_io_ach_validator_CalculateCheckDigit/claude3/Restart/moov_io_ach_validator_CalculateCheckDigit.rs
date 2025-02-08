
use std::char;

const MOOV_IO_ACH_GLPRENOTEDEBIT: i32 = 48;

struct MoovIoAchValidator {}

impl MoovIoAchValidator {
    fn calculate_check_digit(&self, routing_number: &str) -> i32 {
        moov_io_ach_calculate_check_digit(routing_number)
    }
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

        let n = c.to_digit(10).unwrap();

        match i {
            0 | 3 | 6 => sum += n as i32 * 3,
            1 | 4 | 7 => sum += n as i32 * 7,
            2 | 5 => sum += n as i32,
            _ => {}
        }
    }

    moov_io_ach_round_up10(sum) - sum
}

fn moov_io_ach_round_up10(n: i32) -> i32 {
    let ceil = (n as f64 / 10.0).ceil() * 10.0;
    ceil as i32
}
