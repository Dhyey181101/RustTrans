

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = get_pad_string(m as usize);
            return pad.to_string() + &s;
        }
    }
}

struct MoovIoAchADVBatchControl {
    batch_number: i32,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchADVBatchControl {
    fn batch_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.batch_number, 7)
    }
}

struct MoovIoAchStringZeros {
    map: HashMap<usize, String>,
}

impl MoovIoAchStringZeros {
    fn new(max: usize, zero: &str) -> MoovIoAchStringZeros {
        let mut map = HashMap::new();
        for i in 0..=max {
            let pad = get_pad_string(i);
            map.insert(i, pad);
        }
        MoovIoAchStringZeros { map }
    }

    fn get_pad_string(&self, n: usize) -> String {
        match self.map.get(&n) {
            Some(pad) => pad.clone(),
            None => "0".repeat(n),
        }
    }
}

fn get_pad_string(n: usize) -> String {
    "0".repeat(n)
}

impl fmt::Display for MoovIoAchADVBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{
            batch_number: {},
            moov_io_ach_converters: {{}}
        }}",
            self.batch_number
        )
    }
}

fn main() {
    let moov_io_ach_string_zeros = Box::new(MoovIoAchStringZeros::new(94, ZEROS));
    let moov_io_ach_converters = Box::new(MoovIoAchConverters);
    let moov_io_ach_adv_batch_control = Box::new(MoovIoAchADVBatchControl {
        batch_number: 12345,
        moov_io_ach_converters: *moov_io_ach_converters,
    });
    println!("{}", moov_io_ach_adv_batch_control.batch_number_field());
}

