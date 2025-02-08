

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchEntryDetail {
    identifier_number: String,
    // ... other fields elided ...
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters;

impl MoovIoAchEntryDetail {
    fn shr_document_reference_number_field(&self) -> String {
        self.string_field(&self.identifier_number[4..15], 11)
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let mut result = String::with_capacity(max as usize);
        let mut chars = s.chars();
        for c in chars.take(max as usize) {
            result.push(c);
        }
        result
    }
}

