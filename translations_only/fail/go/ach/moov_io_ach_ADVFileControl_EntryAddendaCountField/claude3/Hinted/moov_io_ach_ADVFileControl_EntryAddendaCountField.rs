
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAdvFileControl {
    entry_addenda_count: u64,
}

impl MoovIoAchAdvFileControl {
    fn entry_addenda_count_field(&self) -> String {
        self.numeric_field(self.entry_addenda_count, 8)
    }

    fn numeric_field(&self, n: u64, max: u8) -> String {
        let s = n.to_string();
        if s.len() > max as usize {
            return String::from(&s[(s.len() - max as usize)..]);
        } else {
            let m = (max as usize) - s.len();
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS
                    .as_ref()
                    .unwrap_or_else(|| {
                        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
                        MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap()
                    })
                    .get(&m)
                    .unwrap_or(&String::new())
                    .clone()
            };
            pad + &s
        }
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }

    let test_cases = [
        (4663397149793384447, "93384447"),
        (443323122685378559, "85378559"),
        (18446744035054845696, "Input is invalid, crash gracefully"),
        (17870001846445670399, "Input is invalid, crash gracefully"),
    ];

    for (entry_addenda_count, expected_output) in test_cases {
        let file_control = Box::new(MoovIoAchAdvFileControl {
            entry_addenda_count,
        });
        let result = file_control.entry_addenda_count_field();
        if result == expected_output {
            println!("Test case passed");
        } else {
            println!(
                "Test case failed. Expected: {}, Got: {}",
                expected_output, result
            );
        }
    }
}
