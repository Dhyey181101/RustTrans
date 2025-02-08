
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

fn moov_io_ach_entry_detail_shr_document_reference_number_field(ed: &Box<MoovIoAchEntryDetail>) -> String {
    moov_io_ach_converters_string_field(&ed.identification_number[4..15], 11)
}

fn moov_io_ach_converters_string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s[..max as usize].to_string();
    }

    let m = (max - ln) as usize;
    MOOV_IO_ACH_STRING_ZEROS.get(&m).map_or_else(|| "0".repeat(m) + s, |pad| pad.to_string() + s)
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchEntryDetail {
    identification_number: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {}
