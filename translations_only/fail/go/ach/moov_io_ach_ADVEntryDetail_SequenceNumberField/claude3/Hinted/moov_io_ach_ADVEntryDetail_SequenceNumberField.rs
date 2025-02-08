
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_adv_entry_detail_sequence_number_field(ed: &Box<moov_io_ach_ADVEntryDetail>) -> String {
    moov_io_ach_numeric_field(ed.sequence_number, 4)
}

fn moov_io_ach_numeric_field(n: u64, max: u32) -> String {
    let s = n.to_string();
    let l = s.len();
    if l > max as usize {
        s.chars().rev().take(max as usize).collect()
    } else {
        let m = (max as usize) - l;
        unsafe {
            if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
            }
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m) {
                pad.to_owned() + &s
            } else {
                "0".repeat(m) + &s
            }
        }
    }
}

struct moov_io_ach_ADVEntryDetail {
    sequence_number: u64,
}

struct moov_io_ach_converters;
