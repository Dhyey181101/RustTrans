
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<i32, String>>> = None;

struct MoovIoAchAddenda99 {
    addenda_information: String,
}

impl MoovIoAchAddenda99 {
    fn iat_payment_amount(&mut self, s: &str) {
        let ln = s.chars().count() as u32;
        let max = 10;
        if ln > max {
            self.addenda_information = s.chars().take(max as usize).collect();
        } else {
            let m = (max - ln) as i32;
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS
                    .as_ref()
                    .unwrap()
                    .get(&m)
                    .unwrap_or(&String::new())
                    .clone()
            };
            self.addenda_information = format!("{}{}", pad, s);
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out: HashMap<i32, String> = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(Box::new(moov_io_ach_populate_map(94, "0")));
    }
}
