
use std::fmt;
use std::iter;
use std::collections::HashMap;
use lazy_static;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADDENDA_POS: &str = "7";

fn moov_io_ach_populate_map(max: usize, zero: char) -> HashMap<usize, String> {
let mut out: HashMap<usize, String> = HashMap::new();
for i in 0..max {
out.insert(i, iter::repeat(zero).take(i).collect());
}
out
}
