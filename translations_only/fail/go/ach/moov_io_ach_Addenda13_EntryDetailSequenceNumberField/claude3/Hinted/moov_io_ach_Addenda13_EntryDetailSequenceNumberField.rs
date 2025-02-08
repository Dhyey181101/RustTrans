
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAddenda13 {
    entry_detail_sequence_number: u64,
}

impl MoovIoAchAddenda13 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: u64, max: u8) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            return s[(l - max as usize)..].to_string();
        } else {
            let m = (max as usize) - l;
            unsafe {
                if let Some(ref map) = MOOV_IO_ACH_STRING_ZEROS {
                    if let Some(pad) = map.get(&m) {
                        return pad.to_owned() + &s;
                    }
                } else {
                    MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(95, "0"));
                }
            }
            "0".repeat(m) + &s
        }
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(95, "0"));
    }

    let addenda13 = Box::new(MoovIoAchAddenda13 {
        entry_detail_sequence_number: 4611715213694963199,
    });
    println!("{}", addenda13.entry_detail_sequence_number_field());

    let addenda13 = Box::new(MoovIoAchAddenda13 {
        entry_detail_sequence_number: 721138889812482570,
    });
    println!("{}", addenda13.entry_detail_sequence_number_field());

    let addenda13 = Box::new(MoovIoAchAddenda13 {
        entry_detail_sequence_number: 10199964367928164352,
    });
    println!("{}", addenda13.entry_detail_sequence_number_field());

    let addenda13 = Box::new(MoovIoAchAddenda13 {
        entry_detail_sequence_number: 14771807877286854144,
    });
    println!("{}", addenda13.entry_detail_sequence_number_field());
}
