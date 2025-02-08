
use std::ops::Range;

fn calculate_check_digit(routing_number: &str) -> i32 {
    if routing_number.chars().count() != 8 && routing_number.chars().count() != 9 {
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

        match i {
            0 | 3 | 6 => sum += n * 3,
            1 | 4 | 7 => sum += n * 7,
            2 | 5 => sum += n,
            _ => (),
        }
    }

    round_up_10(sum) - sum
}

fn round_up_10(n: i32) -> i32 {
    ((n as f64 / 10.0).ceil() * 10.0) as i32
}
