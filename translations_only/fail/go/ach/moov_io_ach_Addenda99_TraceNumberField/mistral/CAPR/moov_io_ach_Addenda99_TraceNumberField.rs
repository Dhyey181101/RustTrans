

use std::collections::HashMap;
use std::fmt;
use lazy_static::lazy_static;

struct MoovIoAchConverters {
moov_io_ach_string_zeros: HashMap<usize, String>,
}

impl MoovIoAchConverters {
fn new() -> Self {
MoovIoAchConverters {
moov_io_ach_string_zeros: MoovIoAchConverters::moov_io_ach_populate_map(MOOV_IO_ACH_MAX, "0"),
}
}

fn string_field(&self, s: &str, max: usize) -> String {
let ln = s.len();
if ln > max {
s[..max].to_string()
} else {
let m = max - ln;
let pad = self
.moov_io_ach_string_zeros
.get(&m)
.cloned()
.unwrap_or_else(|| "0".repeat(m));
pad + s
}
}
}

impl MoovIoAchConverters {
fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
let mut out = HashMap::new();
for i in 0..max {
out.insert(i, zero.repeat(i));
}
out
}
}

const MOOV_IO_ACH_MAX: usize = 94;

lazy_static! {
static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = {
MoovIoAchConverters::new().moov_io_ach_string_zeros
};
}

struct MoovIoAchAddenda99 {
trace_number: String,
moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda99 {
fn trace_number_field(&self) -> String {
self.moov_io_ach_converters.string_field(&self.trace_number, 15)
}
}

impl fmt::Display for MoovIoAchAddenda99 {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
write!(
f,
"TraceNumber: {}",
self.trace_number
)
}
}

struct MoovIoAchAddenda99Wrapper {
moov_io_ach_addenda99: MoovIoAchAddenda99,
}

impl MoovIoAchAddenda99Wrapper {
fn trace_number_field(&self) -> String {
self.moov_io_ach_addenda99.trace_number_field()
}
}

impl fmt::Display for MoovIoAchAddenda99Wrapper {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
write!(
f,
"{}",
self.moov_io_ach_addenda99
)
}
}

