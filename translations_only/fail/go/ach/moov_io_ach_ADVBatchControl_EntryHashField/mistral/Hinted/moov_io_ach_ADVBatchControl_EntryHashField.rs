

use std::collections::HashMap;
use std::fmt;
use std::str;

#[derive(Default)]
struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = MoovIoAchStringZeros::new().get_pad_string(m as usize);
            return pad + &s;
        }
    }
}

struct MoovIoAchStringZeros {
    data: HashMap<usize, String>,
}

impl Default for MoovIoAchStringZeros {
    fn default() -> Self {
        let mut data = HashMap::new();
        for i in 0..94 {
            data.insert(i, "0".repeat(i));
        }
        MoovIoAchStringZeros { data }
    }
}

impl MoovIoAchStringZeros {
    fn new() -> Self {
        MoovIoAchStringZeros::default()
    }

    fn get_pad_string(&self, n: usize) -> String {
        match self.data.get(&n) {
            Some(s) => s.clone(),
            None => "0".repeat(n),
        }
    }
}

struct MoovIoAchAdvBatchControl {
    entry_hash: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvBatchControl {
    fn entry_hash_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_hash, 10)
    }
}

struct EntryHashFieldValidator;

impl fmt::Display for EntryHashFieldValidator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EntryHashFieldValidator")
    }
}

fn main() {
    let mut bc = MoovIoAchAdvBatchControl {
        entry_hash: 1848480111,
        moov_io_ach_converters: Box::new(MoovIoAchConverters),
    };
    println!("{}", bc.entry_hash_field());
}

#[test]
fn test_entry_hash_field() {
    let test_cases = vec![
        (1848480111, 4285403758),
        (-309264147, 177312000),
        (-1895825409, 4294967295),
        (797895936, 168430225),
    ];

    for (input_entry_hash, input_max) in test_cases {
        let mut bc = MoovIoAchAdvBatchControl {
            entry_hash: input_entry_hash,
            moov_io_ach_converters: Box::new(MoovIoAchConverters),
        };
        let expected_output = format!("{}", input_entry_hash);
        assert_eq!(expected_output, bc.entry_hash_field());
    }
}

