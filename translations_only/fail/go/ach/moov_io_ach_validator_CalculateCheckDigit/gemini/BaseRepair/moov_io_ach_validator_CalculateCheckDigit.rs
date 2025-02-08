
use std::ops::Range;

pub fn calculate_check_digit(routing_number: &str) -> i32 {
    if routing_number.len() != 8 && routing_number.len() != 9 {
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
            0 | 3 | 6 => sum += (n * 3) as i32,
            1 | 4 | 7 => sum += (n * 7) as i32,
            2 | 5 => sum += n as i32,
            _ => (),
        }
    }

    round_up_10(sum) - sum
}

fn round_up_10(n: i32) -> i32 {
    let range = 0..10;
    let mut result = n;
    if !range.contains(&result) {
        result = (n / 10 + 1) * 10;
    }
    result
}
