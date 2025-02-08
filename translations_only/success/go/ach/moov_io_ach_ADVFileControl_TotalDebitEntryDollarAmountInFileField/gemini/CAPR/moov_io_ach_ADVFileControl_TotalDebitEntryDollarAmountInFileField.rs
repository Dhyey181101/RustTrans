
use std::collections::HashMap;

pub struct MoovIoAchAdvFileControl {
    pub total_debit_entry_dollar_amount_in_file: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAdvFileControl {
    pub fn total_debit_entry_dollar_amount_in_file_field(&self) -> String {
        let mut pad = String::new();
        let m = 20 - self.total_debit_entry_dollar_amount_in_file.to_string().len() as u32;
        if m > 0 {
            pad = moov_io_ach_string_zeros().get(&m).unwrap().to_string();
        }
        format!("{}{}", pad, self.total_debit_entry_dollar_amount_in_file)
    }
}

impl MoovIoAchConverters {}

fn moov_io_ach_string_zeros() -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, String::from("0").repeat(i as usize));
    }
    out
}
