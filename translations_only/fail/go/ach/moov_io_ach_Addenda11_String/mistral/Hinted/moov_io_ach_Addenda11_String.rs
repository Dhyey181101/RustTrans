
use std::fmt;
use std::collections::HashMap;
use lazy_static;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADENDA_POS: &str = "7";

fn moov_io_ach_populate_map(max: usize, zero: char) -> HashMap<usize, String> {
let mut out = HashMap::new();
for i in 0..max {
out.insert(i, "".repeat(i as usize).chars().map(|_| zero).collect());
}
out
}
