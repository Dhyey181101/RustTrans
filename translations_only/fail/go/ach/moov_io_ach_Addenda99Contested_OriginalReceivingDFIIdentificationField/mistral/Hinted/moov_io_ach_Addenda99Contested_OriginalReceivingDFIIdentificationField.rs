

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

#[derive(Debug)]
struct Converters;

struct Addenda99Contested {
    original_receiving_dfi_identification: String,
    moov_io_ach_converters: Box<Converters>,
}

impl Addenda99Contested {
    fn new(
        original_receiving_dfi_identification: String,
        moov_io_ach_converters: Converters,
    ) -> Self {
        Self {
            original_receiving_dfi_identification,
            moov_io_ach_converters: Box::new(moov_io_ach_converters),
        }
    }
}

impl fmt::Display for Addenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Original Receiving DFI Identification: {}, Moov IO ACH Converters: {:?}",
            self.original_receiving_dfi_identification, self.moov_io_ach_converters
        )
    }
}

#[derive(Debug)]
struct Addenda99Uncontested {
    addenda99_uncontested_information: String,
}

struct Addenda99 {
    addenda99_contested: Addenda99Contested,
    addenda99_uncontested: Addenda99Uncontested,
}

impl Addenda99 {
    fn new(
        addenda99_contested: Addenda99Contested,
        addenda99_uncontested: Addenda99Uncontested,
    ) -> Self {
        Self {
            addenda99_contested,
            addenda99_uncontested,
        }
    }
}

impl fmt::Display for Addenda99 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\nAddenda99Contested: {},\nAddenda99Uncontested: {}",
            self.addenda99_contested, self.addenda99_uncontested.addenda99_uncontested_information
        )
    }
}

fn main() {
    let addenda99_contested = Addenda99Contested::new(
        "123456789".to_string(),
        Converters,
    );

    let addenda99_uncontested = Addenda99Uncontested {
        addenda99_uncontested_information: "Some Information".to_string(),
    };

    let addenda99 = Addenda99::new(addenda99_contested, addenda99_uncontested);

    println!("{}", addenda99);
}

