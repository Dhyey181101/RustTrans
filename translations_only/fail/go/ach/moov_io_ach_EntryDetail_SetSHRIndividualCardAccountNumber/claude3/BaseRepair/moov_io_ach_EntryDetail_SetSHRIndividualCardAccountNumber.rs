
extern crate lazy_static;

use std::collections::HashMap;
use std::str;
use lazy_static::lazy_static;

const CHECKING_CREDIT: usize = 22;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = populate_map(94, "0".to_string());
}

fn set_shr_individual_card_account_number(ed: &mut Box<EntryDetail>, s: &str) {
    ed.individual_name = string_field(s, 22);
}

fn string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        return s.chars().take(max).collect();
    }

    let m = max - ln;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return pad.to_owned() + s;
    }

    // slow path
    "0".repeat(m) + s
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct EntryDetail {
    individual_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_shr_individual_card_account_number() {
        let mut ed = Box::new(EntryDetail {
            individual_name: String::new(),
        });
        set_shr_individual_card_account_number(&mut ed, "John Doe");
        assert_eq!(ed.individual_name, "John Doe0000000000000");
    }
}
