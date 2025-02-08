
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

const MOO_IO_ACH_RECORD_LENGTH: usize = 94;
const MOO_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Debug, Default)]
pub struct MoovIoAchAddenda05 {
    pub type_code: String,
    pub payment_related_information: String,
    pub sequence_number: i32,
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda05 {
    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(MOO_IO_ACH_RECORD_LENGTH);
        write!(buf, "{}", MOO_IO_ACH_ENTRY_ADDENDA_POS).unwrap();
        write!(buf, "{}", self.type_code).unwrap();
        write!(buf, "{}", self.payment_related_information_field()).unwrap();
        write!(buf, "{}", self.sequence_number_field()).unwrap();
        write!(buf, "{}", self.entry_detail_sequence_number_field()).unwrap();
        buf
    }

    pub fn payment_related_information_field(&self) -> String {
        self.alpha_field(&self.payment_related_information, 80)
    }

    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = SPACE_ZEROS.get(&m).unwrap();
            format!("{}{}", s, pad)
        }
    }

    pub fn sequence_number_field(&self) -> String {
        self.numeric_field(self.sequence_number, 4)
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.chars().count();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
            let pad = STRING_ZEROS.get(&m).unwrap();
            format!("{}{}", pad, s)
        }
    }

    pub fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

lazy_static::lazy_static! {
    static ref SPACE_ZEROS: HashMap<usize, String> = populate_map(94, " ");
    static ref STRING_ZEROS: HashMap<usize, String> = populate_map(94, "0");
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
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
        let addenda05 = MoovIoAchAddenda05 {
            type_code: "05".to_string(),
            payment_related_information: "Payment Related Information".to_string(),
            sequence_number: 1,
            entry_detail_sequence_number: 1234567,
        };
        assert_eq!(
            addenda05.to_string(),
            "705Payment Related Information00011234567"
        );
    }

    #[test]
    fn test_payment_related_information_field() {
        let addenda05 = MoovIoAchAddenda05 {
            payment_related_information: "Payment Related Information".to_string(),
            ..Default::default()
        };
        assert_eq!(
            addenda05.payment_related_information_field(),
            "Payment Related Information                                "
        );
    }

    #[test]
    fn test_alpha_field() {
        let addenda05 = MoovIoAchAddenda05 {
            payment_related_information: "Payment Related Information".to_string(),
            ..Default::default()
        };
        assert_eq!(
            addenda05.alpha_field(&addenda05.payment_related_information, 80),
            "Payment Related Information                                "
        );
    }

    #[test]
    fn test_sequence_number_field() {
        let addenda05 = MoovIoAchAddenda05 {
            sequence_number: 1,
            ..Default::default()
        };
        assert_eq!(addenda05.sequence_number_field(), "0001");
    }

    #[test]
    fn test_numeric_field() {
        let addenda05 = MoovIoAchAddenda05 {
            sequence_number: 1,
            ..Default::default()
        };
        assert_eq!(addenda05.numeric_field(addenda05.sequence_number, 4), "0001");
    }

    #[test]
    fn test_entry_detail_sequence_number_field() {
        let addenda05 = MoovIoAchAddenda05 {
            entry_detail_sequence_number: 1234567,
            ..Default::default()
        };
        assert_eq!(
            addenda05.entry_detail_sequence_number_field(),
            "1234567"
        );
    }
}
