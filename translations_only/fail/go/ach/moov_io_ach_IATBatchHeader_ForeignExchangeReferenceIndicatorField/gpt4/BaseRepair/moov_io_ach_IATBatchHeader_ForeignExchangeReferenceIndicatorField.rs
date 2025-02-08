
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchIATBatchHeader {
    foreign_exchange_reference_indicator: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchIATBatchHeader {
    fn foreign_exchange_reference_indicator_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.foreign_exchange_reference_indicator, 1)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s.chars().skip((l - max) as usize).collect()
        } else {
            let m = (max - l) as i32;
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m).unwrap_or(&String::new()).clone()
            };
            pad + &s
        }
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
    let converters = Box::new(MoovIoAchConverters {});
    let iat_batch_header = MoovIoAchIATBatchHeader {
        foreign_exchange_reference_indicator: 3,
        moov_io_ach_converters: converters,
    };
    println!("{}", iat_batch_header.foreign_exchange_reference_indicator_field());
}
