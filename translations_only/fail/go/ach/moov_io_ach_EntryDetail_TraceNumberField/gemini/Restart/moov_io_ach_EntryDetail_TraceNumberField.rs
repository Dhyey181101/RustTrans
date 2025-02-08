
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchEntryDetail {
    pub trace_number: String,
    pub addenda_02: Option<String>,
    pub addenda_05: Option<String>,
    pub addenda_98: Option<String>,
    pub addenda_98_for_user_with_refused_notification_of_change: Option<String>,
    pub addenda_99: Option<String>,
    pub addenda_99_contested: Option<String>,
    pub addenda_99_dishonored: Option<String>,
    pub category: String,
}

impl MoovIoAchEntryDetail {
    pub fn trace_number_field(&self) -> String {
        self.string_field(&self.trace_number, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let mut pad = String::with_capacity(max - ln);
            for _ in 0..(max - ln) {
                pad.push('0');
            }
            pad + s
        }
    }
}

pub fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

#[derive(Debug)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let mut pad = String::with_capacity(max - ln);
            for _ in 0..(max - ln) {
                pad.push('0');
            }
            pad + s
        }
    }
}

pub fn moov_io_ach_entry_detail_from_str(s: &str) -> Result<MoovIoAchEntryDetail, String> {
    let mut entry_detail = MoovIoAchEntryDetail {
        trace_number: String::new(),
        addenda_02: None,
        addenda_05: None,
        addenda_98: None,
        addenda_98_for_user_with_refused_notification_of_change: None,
        addenda_99: None,
        addenda_99_contested: None,
        addenda_99_dishonored: None,
        category: String::new(),
    };

    let mut lines = s.lines();
    while let Some(line) = lines.next() {
        if line.starts_with("TraceNumber") {
            entry_detail.trace_number = line[11..].trim().to_string();
        } else if line.starts_with("Addenda02") {
            entry_detail.addenda_02 = Some(line[9..].trim().to_string());
        } else if line.starts_with("Addenda05") {
            entry_detail.addenda_05 = Some(line[9..].trim().to_string());
        } else if line.starts_with("Addenda98") {
            entry_detail.addenda_98 = Some(line[9..].trim().to_string());
        } else if line.starts_with("Addenda98") {
            entry_detail.addenda_98_for_user_with_refused_notification_of_change =
                Some(line[9..].trim().to_string());
        } else if line.starts_with("Addenda99") {
            entry_detail.addenda_99 = Some(line[9..].trim().to_string());
        } else if line.starts_with("Addenda99") {
            entry_detail.addenda_99_contested = Some(line[9..].trim().to_string());
        } else if line.starts_with("Addenda99") {
            entry_detail.addenda_99_dishonored = Some(line[9..].trim().to_string());
        } else if line.starts_with("Category") {
            entry_detail.category = line[8..].trim().to_string();
        }
    }

    Ok(entry_detail)
}
