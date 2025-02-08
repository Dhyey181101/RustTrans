
use std::ops::Add;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_check_digit() {
        assert_eq!(calculate_check_digit("12345678"), 0);
        assert_eq!(calculate_check_digit("123456789"), 9);
        assert_eq!(calculate_check_digit("123456780"), -1);
        assert_eq!(calculate_check_digit("12345678a"), -1);
    }

    #[test]
    fn test_round_up_10() {
        assert_eq!(round_up_10(0), 0);
        assert_eq!(round_up_10(1), 10);
        assert_eq!(round_up_10(9), 10);
        assert_eq!(round_up_10(10), 10);
        assert_eq!(round_up_10(11), 20);
        assert_eq!(round_up_10(19), 20);
        assert_eq!(round_up_10(20), 20);
        assert_eq!(round_up_10(21), 30);
        assert_eq!(round_up_10(29), 30);
        assert_eq!(round_up_10(30), 30);
        assert_eq!(round_up_10(31), 40);
        assert_eq!(round_up_10(39), 40);
        assert_eq!(round_up_10(40), 40);
        assert_eq!(round_up_10(41), 50);
        assert_eq!(round_up_10(49), 50);
        assert_eq!(round_up_10(50), 50);
        assert_eq!(round_up_10(51), 60);
        assert_eq!(round_up_10(59), 60);
        assert_eq!(round_up_10(60), 60);
        assert_eq!(round_up_10(61), 70);
        assert_eq!(round_up_10(69), 70);
        assert_eq!(round_up_10(70), 70);
        assert_eq!(round_up_10(71), 80);
        assert_eq!(round_up_10(79), 80);
        assert_eq!(round_up_10(80), 80);
        assert_eq!(round_up_10(81), 90);
        assert_eq!(round_up_10(89), 90);
        assert_eq!(round_up_10(90), 90);
        assert_eq!(round_up_10(91), 100);
        assert_eq!(round_up_10(99), 100);
        assert_eq!(round_up_10(100), 100);
        assert_eq!(round_up_10(101), 110);
        assert_eq!(round_up_10(109), 110);
        assert_eq!(round_up_10(110), 110);
        assert_eq!(round_up_10(111), 120);
        assert_eq!(round_up_10(119), 120);
        assert_eq!(round_up_10(120), 120);
        assert_eq!(round_up_10(121), 130);
        assert_eq!(round_up_10(129), 130);
        assert_eq!(round_up_10(130), 130);
        assert_eq!(round_up_10(131), 140);
        assert_eq!(round_up_10(139), 140);
        assert_eq!(round_up_10(140), 140);
        assert_eq!(round_up_10(141), 150);
        assert_eq!(round_up_10(149), 150);
        assert_eq!(round_up_10(150), 150);
        assert_eq!(round_up_10(151), 160);
        assert_eq!(round_up_10(159), 160);
        assert_eq!(round_up_10(160), 160);
        assert_eq!(round_up_10(161), 170);
        assert_eq!(round_up_10(169), 170);
        assert_eq!(round_up_10(170), 170);
        assert_eq!(round_up_10(171), 180);
        assert_eq!(round_up_10(179), 180);
        assert_eq!(round_up_10(180), 180);
        assert_eq!(round_up_10(181), 190);
        assert_eq!(round_up_10(189), 190);
        assert_eq!(round_up_10(190), 190);
        assert_eq!(round_up_10(191), 200);
        assert_eq!(round_up_10(199), 200);
        assert_eq!(round_up_10(200), 200);
    }
}
