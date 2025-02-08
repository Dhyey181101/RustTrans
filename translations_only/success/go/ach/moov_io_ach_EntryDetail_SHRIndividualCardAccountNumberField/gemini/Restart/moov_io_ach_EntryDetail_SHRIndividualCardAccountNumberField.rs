
use lazy_static::lazy_static;
use std::collections::HashMap;

const MOOv_IO_ACH_CHECKING_CREDIT: u8 = 22;

pub struct MoovIoAchEntryDetail {
    pub individual_name: String,
}

impl MoovIoAchEntryDetail {
    pub fn shr_individual_card_account_number_field(&self) -> String {
        let ln = self.individual_name.chars().count() as u32;
        if ln > MOOv_IO_ACH_CHECKING_CREDIT as u32 {
            return self.individual_name[..MOOv_IO_ACH_CHECKING_CREDIT as usize].to_string();
        }

        let m = MOOv_IO_ACH_CHECKING_CREDIT as u32 - ln;
        let pad = MOOv_IO_ACH_STRING_ZEROS.get(&m).unwrap();
        return format!("{}{}", pad, self.individual_name);
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
        let pad = MOOv_IO_ACH_STRING_ZEROS.get(&m).unwrap();
        return format!("{}{}", pad, s);
    }
}

lazy_static! {
    static ref MOOv_IO_ACH_STRING_ZEROS: HashMap<u32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}
