
use std::fmt;
use std::collections::HashMap;
use std::convert::TryInto;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchAdvFileControl {
    entry_addenda_count: i32,
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    numeric_field_max: u32,
}
