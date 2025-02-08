
use std::f64;

struct MoovIoAchValidator;

impl MoovIoAchValidator {
    fn calculate_check_digit(&self, routing_number: &str) -> i32 {
        moov_io_ach_calculate_check_digit(routing_number)
    }
}

fn moov_io_ach_calculate_check_digit(routing_number: &str) -> i32 {
    let n = routing_number.chars().count();
    if n != 8 && n != 9 {
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

    moov_io_ach_round_up_10(sum) - sum
}

fn moov_io_ach_round_up_10(n: i32) -> i32 {
    ((f64::from(n) / 10.0).ceil() * 10.0) as i32
}
