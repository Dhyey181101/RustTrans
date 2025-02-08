

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Converters;

impl Converters {
// add methods here if needed
}

struct Addenda99Contested {
original_receiving_dfi_identification: String,
moov_io_ach_converters: Box<Converters>,
}

impl Addenda99Contested {
fn new(
original_receiving_dfi_identification: String,
moov_io_ach_converters: Converters,
) -> Self {
Self {
original_receiving_dfi_identification,
moov_io_ach_converters: Box::new(moov_io_ach_converters),
}
}
}

