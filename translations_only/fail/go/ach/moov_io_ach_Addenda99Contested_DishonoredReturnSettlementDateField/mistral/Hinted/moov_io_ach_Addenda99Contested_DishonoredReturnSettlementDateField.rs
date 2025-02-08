

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

impl fmt::Display for Addenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DISHONORED RETURN SETTLEMENT DATE:{}{}",
            ZERO, self.dishonored_return_settlement_date
        )
    }
}

