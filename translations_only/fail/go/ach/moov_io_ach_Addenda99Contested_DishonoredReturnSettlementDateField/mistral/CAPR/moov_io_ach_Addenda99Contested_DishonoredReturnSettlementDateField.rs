
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99Contested {
    dishonored_return_settlement_date: String,
}

impl Addenda99Contested {
    fn new(dishonored_return_settlement_date: String) -> Box<Addenda99Contested> {
        Box::new(Addenda99Contested {
            dishonored_return_settlement_date,
        })
    }
}
