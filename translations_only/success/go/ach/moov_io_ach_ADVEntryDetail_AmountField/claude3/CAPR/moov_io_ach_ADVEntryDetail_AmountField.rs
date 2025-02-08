

use std::collections::HashMap;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

fn moov_io_ach_adv_entry_detail_amount_field(ed: &Box<moov_io_ach_ADVEntryDetail>) -> String {
    moov_io_ach_numeric_field(ed.amount, 12)
}

fn moov_io_ach_numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    let l = s.len();
    if l > max as usize {
        s.chars().rev().take(max as usize).collect()
    } else {
        let m = (max as usize) - l;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&String::new()).to_owned();
        pad + &s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct moov_io_ach_ADVEntryDetail {
    amount: i32,
    moov_io_ach_converters: Box<moov_io_ach_converters>,
}

struct moov_io_ach_converters {}

use once_cell::sync::Lazy;

