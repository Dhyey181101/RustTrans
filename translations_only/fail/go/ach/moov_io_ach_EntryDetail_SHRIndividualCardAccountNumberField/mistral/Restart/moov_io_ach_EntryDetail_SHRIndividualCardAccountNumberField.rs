

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';
const CHECKING_CREDIT: u32 = 22;

struct EntryDetail {
    individual_name: String,
    // ... other fields omitted for brevity
    converters: Box<Converters>,
}

struct Converters;

impl Converters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln: usize = s.chars().count();
        let mut result = String::with_capacity(s.len());

        if ln > (max as usize) {
            result.push_str(&s[..(max as usize)]);
        } else {
            result.push_str(s);

            let m: usize = (max as usize) - ln;
            result.extend(std::iter::repeat(ZERO).take(m));
        }

        result
    }
}

impl EntryDetail {
    fn shr_individual_card_account_number_field(&self) -> String {
        self.converters.string_field(&self.individual_name, CHECKING_CREDIT)
    }
}

fn populate_map(max: usize, zero: char) -> HashMap<usize, String> {
    let mut out: HashMap<usize, String> = HashMap::new();

    for i in 0..max {
        out.insert(i, (0..i + 1).map(|_| zero).collect::<String>());
    }

    out
}

impl fmt::Display for EntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implementation omitted for brevity
        write!(f, "{}", self.individual_name)
    }
}

