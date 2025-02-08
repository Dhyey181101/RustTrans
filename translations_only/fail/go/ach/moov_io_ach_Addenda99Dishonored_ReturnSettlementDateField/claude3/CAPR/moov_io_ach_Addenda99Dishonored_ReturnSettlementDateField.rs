

use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, Box<String>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, Box<String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i)));
    }
    out
}

fn moov_io_ach_string_zeros() -> &'static HashMap<usize, Box<String>> {
    unsafe {
        if MOOV_IO_ACH_STRING_ZEROS.is_none() {
            MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        }
        MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap()
    }
}

impl Addenda99Dishonored {
    fn return_settlement_date_field(&self) -> String {
        self.converters.string_field(self.return_settlement_date.as_ref(), 3)
    }
}

impl Converters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        let m = (max - ln) as usize;
        let pad = moov_io_ach_string_zeros().get(&m).map_or(Box::new(String::new()), |x| x.clone());
        pad.to_string() + s
    }
}

struct Addenda99Dishonored {
    return_settlement_date: Box<String>,
    converters: Converters,
}

struct Converters {}

