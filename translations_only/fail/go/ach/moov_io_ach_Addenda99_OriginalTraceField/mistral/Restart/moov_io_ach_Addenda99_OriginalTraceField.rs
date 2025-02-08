

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99 {
    original_trace: String,
    moov_io_ach_converters: Converters,
}

struct Converters;

impl Addenda99 {
    fn new(original_trace: String, moov_io_ach_converters: Converters) -> Addenda99 {
        Addenda99 {
            original_trace,
            moov_io_ach_converters,
        }
    }
}

struct Entry {
    sequence: i32,
    amount: i32,
}

impl Entry {
    fn new(sequence: i32, amount: i32) -> Entry {
        Entry { sequence, amount }
    }
}

struct Record {
    addenda: Vec<Entry>,
}

impl Record {
    fn new() -> Record {
        Record { addenda: Vec::new() }
    }

    fn add(&mut self, entry: Entry) {
        self.addenda.push(entry);
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for (index, entry) in self.addenda.iter().enumerate() {
            result.push_str(&format!(
                "{}{:02}{:012}{:012}",
                ZERO,
                index + 1,
                entry.sequence,
                entry.amount
            ));
        }
        result
    }
}

struct Trace {
    records: Vec<Record>,
}

impl Trace {
    fn new() -> Trace {
        Trace { records: Vec::new() }
    }

    fn add(&mut self, record: Record) {
        self.records.push(record);
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for (index, record) in self.records.iter().enumerate() {
            result.push_str(&format!("{:06}{}", index + 1, record.to_string()));
        }
        result
    }
}

struct Addenda9 {
    original_trace: String,
    moov_io_ach_converters: Converters,
}

impl Addenda9 {
    fn new(original_trace: String, moov_io_ach_converters: Converters) -> Addenda9 {
        Addenda9 {
            original_trace,
            moov_io_ach_converters,
        }
    }

    fn convert(&self) -> String {
        let mut trace = Trace::new();
        let mut lines: HashMap<i32, i32> = HashMap::new();
        for line in self.original_trace.lines() {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let sequence = fields[1].parse::<i32>().unwrap();
            let amount = fields[3].parse::<i32>().unwrap();
            lines.insert(sequence, amount);
            if lines.len() > 9 {
                let mut record = Record::new();
                let mut sum = 0;
                for (_, &amount) in lines.iter() {
                    sum += amount;
                }
                record.add(Entry::new(99, sum));
                trace.add(record);
                lines.clear();
            }
        }
        if lines.len() > 0 {
            let mut record = Record::new();
            let mut sum = 0;
            for (_, &amount) in lines.iter() {
                sum += amount;
            }
            record.add(Entry::new(99, sum));
            trace.add(record);
        }
        trace.to_string()
    }
}

impl fmt::Display for Addenda9 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.convert())
    }
}

