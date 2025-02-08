
use std::collections::HashMap;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

fn moov_io_ach_amount_field(ed: &Box<moov_io_ach_EntryDetail>) -> String {
    moov_io_ach_numeric_field(ed.amount, 10)
}

fn moov_io_ach_numeric_field(n: i32, max: u32) -> String {
    let mut s = n.to_string();
    let l = s.len();
    if l > max as usize {
        s.drain(..l - max as usize);
    } else {
        let m = (max as usize) - l;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            s = pad.to_string() + &s;
        } else {
            s = "0".repeat(m) + &s;
        }
    }
    s
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct moov_io_ach_EntryDetail {
    amount: i32,
    moov_io_ach_converters: Box<moov_io_ach_converters>,
}

struct moov_io_ach_converters {}

use once_cell::sync::Lazy;
