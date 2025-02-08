
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

const MOOv_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOv_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda12 {
    pub type_code: String,
    pub originator_city_state_province: String,
    pub originator_country_postal_code: String,
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda12 {
    pub fn new(
        type_code: String,
        originator_city_state_province: String,
        originator_country_postal_code: String,
        entry_detail_sequence_number: i32,
    ) -> Self {
        Self {
            type_code,
            originator_city_state_province,
            originator_country_postal_code,
            entry_detail_sequence_number,
        }
    }

    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(MOOv_IO_ACH_RECORD_LENGTH);
        write!(buf, "{}", MOOv_IO_ACH_ENTRY_ADDENDA_POS).unwrap();
        write!(buf, "{}", self.type_code).unwrap();
        write!(buf, "{}", self.originator_city_state_province_field()).unwrap();
        write!(buf, "{}", self.originator_country_postal_code_field()).unwrap();
        write!(buf, "{: >20}", "").unwrap();
        write!(buf, "{}", self.entry_detail_sequence_number_field()).unwrap();
        buf
    }

    fn originator_city_state_province_field(&self) -> String {
        let mut field = self.originator_city_state_province.clone();
        if field.len() > 35 {
            field = field[..35].to_string();
        }
        field
    }

    fn originator_country_postal_code_field(&self) -> String {
        let mut field = self.originator_country_postal_code.clone();
        if field.len() > 35 {
            field = field[..35].to_string();
        }
        field
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        let mut field = self.entry_detail_sequence_number.to_string();
        if field.len() > 7 {
            field = field[field.len() - 7..].to_string();
        }
        field
    }
}

pub fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let addenda12 = MoovIoAchAddenda12::new(
            "12".to_string(),
            "San Francisco*CA\\".to_string(),
            "US*10036\\".to_string(),
            1234567,
        );
        assert_eq!(
            addenda12.to_string(),
            "712San Francisco*CA\\US*10036\\              1234567"
        );
    }

    #[test]
    fn test_originator_city_state_province_field() {
        let addenda12 = MoovIoAchAddenda12::new(
            "12".to_string(),
            "San Francisco*CA\\".to_string(),
            "US*10036\\".to_string(),
            1234567,
        );
        assert_eq!(addenda12.originator_city_state_province_field(), "San Francisco*CA\\");
    }

    #[test]
    fn test_originator_country_postal_code_field() {
        let addenda12 = MoovIoAchAddenda12::new(
            "12".to_string(),
            "San Francisco*CA\\".to_string(),
            "US*10036\\".to_string(),
            1234567,
        );
        assert_eq!(addenda12.originator_country_postal_code_field(), "US*10036\\");
    }

    #[test]
    fn test_entry_detail_sequence_number_field() {
        let addenda12 = MoovIoAchAddenda12::new(
            "12".to_string(),
            "San Francisco*CA\\".to_string(),
            "US*10036\\".to_string(),
            1234567,
        );
        assert_eq!(addenda12.entry_detail_sequence_number_field(), "1234567");
    }

    #[test]
    fn test_moov_io_ach_populate_map() {
        let map = moov_io_ach_populate_map(94, " ");
        assert_eq!(map.get(&0), Some(&"".to_string()));
        assert_eq!(map.get(&93), Some(&" ".repeat(93)));
    }
}
