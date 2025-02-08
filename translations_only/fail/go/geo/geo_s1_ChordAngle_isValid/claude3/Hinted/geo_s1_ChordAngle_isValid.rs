
use std::f64;

const GEO_S1_MAX_LENGTH2: f64 = 4.0;

fn is_valid(c: f64) -> bool {
    (c >= 0.0 && c <= GEO_S1_MAX_LENGTH2) || is_special(c)
}

fn is_special(c: f64) -> bool {
    c < 0.0 || is_infinity(c)
}

fn is_infinity(c: f64) -> bool {
    c.is_infinite()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid(2.14576952107536e-309));
        assert!(is_valid(1.48539705396e-312));
        assert!(is_valid(f64::NAN));
    }
}
