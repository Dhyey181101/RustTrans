
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_get_string_zeros(max: usize, zero: &str) -> &'static HashMap<usize, String> {
    unsafe {
        if MOOV_IO_ACH_STRING_ZEROS.is_none() {
            MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(max, zero));
        }
        MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap()
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = (max as usize) - l;
            let zeros = moov_io_ach_get_string_zeros(m, "0");
            zeros.get(&m).unwrap().to_owned() + &s
        }
    }
}

struct MoovIoAchAdvBatchControl {
    entry_hash: usize,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvBatchControl {
    fn entry_hash_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_hash, 10)
    }
}
