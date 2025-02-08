
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;

const MOO_IO_ACH_CHECKING_CREDIT: u8 = 22;

#[derive(Debug)]
pub struct MoovIoAchEntryDetail {
    pub individual_name: String,
}

impl MoovIoAchEntryDetail {
    pub fn shr_individual_card_account_number_field(&self) -> String {
        let converters = MoovIoAchConverters {};
        converters.string_field(&self.individual_name, MOO_IO_ACH_CHECKING_CREDIT as u32)
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = MOO_IO_ACH_STRING_ZEROS.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref MOO_IO_ACH_STRING_ZEROS: HashMap<i32, String> =
        (0..94).map(|i| (i, "0".repeat(i as usize))).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_field() {
        let converters = MoovIoAchConverters {};
        assert_eq!(
            converters.string_field("John Doe", 10),
            "John Doe  "
        );
        assert_eq!(converters.string_field("John Doe", 5), "John");
    }

    #[test]
    fn test_shr_individual_card_account_number_field() {
        let entry_detail = MoovIoAchEntryDetail {
            individual_name: "John Doe".to_string(),
        };
        assert_eq!(
            entry_detail.shr_individual_card_account_number_field(),
            "John Doe  "
        );
    }
}
