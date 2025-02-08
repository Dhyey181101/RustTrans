
use std::isize;

fn sum(arr: Vec<isize>) -> isize {
    arr.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_0() {
        let arr = vec![72057593870155776, 721420288, 184683593728];
        assert_eq!(sum(arr), 72057779275169792);
    }

    #[test]
    fn example_1() {
        let arr = vec![-1125899906842624, 255, 0];
        assert_eq!(sum(arr), -1125899906842369);
    }

    #[test]
    fn example_2() {
        let arr = vec![
            3907211571980753664,
            4130039284161328946,
            3185226603239058486,
        ];
        assert_eq!(sum(arr), -7224266614328410520);
    }

    #[test]
    fn example_3() {
        let arr = vec![
            458582, 3472898735913070080, 3975612193140430974, 3689636895445561394, 16325548649217843,
        ];
        assert_eq!(
            sum(arr),
            -7364320598017411529
        );
    }
}
