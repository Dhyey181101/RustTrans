
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99Contested {
    dishonored_return_settlement_date: String,
}

impl Addenda99Contested {
    fn new(dishonored_return_settlement_date: String) -> Addenda99Contested {
        Addenda99Contested {
            dishonored_return_settlement_date,
        }
    }
}

struct Addenda99 {
    addenda99_contested: Option<Box<Addenda99Contested>>,
}

impl Addenda99 {
    fn new() -> Addenda99 {
        Addenda99 {
            addenda99_contested: None,
        }
    }

    fn with_addenda99_contested(addenda99_contested: Addenda99Contested) -> Addenda99 {
        Addenda99 {
            addenda99_contested: Some(Box::new(addenda99_contested)),
        }
    }
}

struct Record {
    addenda99: Addenda99,
}

impl Record {
    fn new() -> Record {
        Record {
            addenda99: Addenda99::new(),
        }
    }
}

struct Records {
    records: HashMap<i32, Record>,
}

impl Records {
    fn new() -> Records {
        Records {
            records: HashMap::new(),
        }
    }

    fn with_record(&mut self, record: Record) {
        self.records.insert(1, record);
    }
}

struct Report {
    records: Records,
}

impl Report {
    fn new() -> Report {
        Report {
            records: Records::new(),
        }
    }

    fn with_records(&mut self, records: Records) {
        self.records = records;
    }
}

fn main() {
    let mut report = Report::new();

    let mut records = Records::new();
    let mut record = Record::new();
    let mut addenda99 = Addenda99::new();
    let addenda99_contested = Addenda99Contested::new("2022-06-01".to_string());
    addenda99 = Addenda99::with_addenda99_contested(addenda99_contested);
    record.addenda99 = addenda99;
}
