
use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters;

const MOOV_IO_ACH_ZERO: &str = "0";

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
let mut out = HashMap::new();
for i in 0..max {
out.insert(i, zero.repeat(i));
}
out
}
