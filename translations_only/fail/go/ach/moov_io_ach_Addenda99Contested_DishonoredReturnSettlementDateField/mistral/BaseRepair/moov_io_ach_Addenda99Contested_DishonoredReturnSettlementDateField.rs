
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

    fn set_addenda99_contested(&mut self, addenda99_contested: Addenda99Contested) {
        self.addenda99_contested = Some(Box::new(addenda99_contested));
    }
}

struct Record99 {
    addenda99: Addenda99,
}

impl Record99 {
    fn new() -> Record99 {
        Record99 {
            addenda99: Addenda99::new(),
        }
    }

    fn set_addenda99(&mut self, addenda99: Addenda99) {
        self.addenda99 = addenda99;
    }
}

struct Record {
    record99: Option<Box<Record99>>,
}

impl Record {
    fn new() -> Record {
        Record {
            record99: None,
        }
    }

    fn set_record99(&mut self, record99: Record99) {
        self.record99 = Some(Box::new(record99));
    }
}

fn main() {}
