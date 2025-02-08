
use std::collections::HashMap;
use std::fmt;
use std::i64;
use std::boxed::Box;

struct MoovIoAchBatchControl {
    batch_number: i64,
}

impl fmt::Display for MoovIoAchBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoovIoAchBatchControl(batch_number: {})", self.batch_number)
    }
}
