
use std::collections::HashMap;
use std::str::FromStr;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_numeric_field(n: usize, max: usize) -> String {
    let s = n.to_string();
    if s.len() > max {
        String::from(&s[s.len() - max..])
    } else {
        let m = max - s.len();
        unsafe {
            if let Some(ref map) = MOOV_IO_ACH_STRING_ZEROS {
                map.get(&m).unwrap_or(&String::new()).to_owned() + &s
            } else {
                "0".repeat(m) + &s
            }
        }
    }
}

struct MoovIoAchBatchControl {
    entry_addenda_count: usize,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchBatchControl {
    fn entry_addenda_count_field(&self) -> String {
        self.moov_io_ach_converters
            .numeric_field(self.entry_addenda_count, 6)
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: usize, max: usize) -> String {
        moov_io_ach_numeric_field(n, max)
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }

    let converters = Box::new(MoovIoAchConverters {});
    let batch_control = MoovIoAchBatchControl {
        entry_addenda_count: 123,
        moov_io_ach_converters: converters,
    };

    println!("{}", batch_control.entry_addenda_count_field());
}
