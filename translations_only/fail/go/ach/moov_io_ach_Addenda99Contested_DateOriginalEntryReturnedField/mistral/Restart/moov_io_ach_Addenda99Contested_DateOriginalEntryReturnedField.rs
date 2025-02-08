
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Converters;

impl Converters {
    // Add any necessary methods here
}

struct Entry {
    r#type: String,
    amount: i32,
}

impl Entry {
    fn new(r#type: &str, amount: i32) -> Entry {
        Entry {
            r#type: r#type.to_string(),
            amount,
        }
    }
}

struct Record {
    entries: Vec<Entry>,
}

impl Record {
    fn new() -> Record {
        Record { entries: Vec::new() }
    }

    fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    fn to_string(&self) -> String {
        let mut record_string = String::new();

        for entry in &self.entries {
            record_string.push_str(&entry.r#type);
            record_string.push_str(&ZERO.to_string());
            record_string.push_str(&(entry.amount).to_string());
        }

        record_string
    }
}

struct Format99 {
    records: Vec<Record>,
}

impl Format99 {
    fn new() -> Format99 {
        Format99 { records: Vec::new() }
    }

    fn add_record(&mut self, record: Record) {
        self.records.push(record);
    }

    fn to_string(&self) -> String {
        let mut format99_string = String::new();

        for record in &self.records {
            format99_string.push_str(&record.to_string());
            format99_string.push('\n');
        }

        format99_string
    }
}
