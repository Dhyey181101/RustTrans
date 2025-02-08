
use std::collections::HashMap;
use std::convert::From;

mod moov_io_ach {
    pub struct Addenda99Contested {
        pub dishonored_return_reason_code: String,
        converters: Converters,
    }

    impl Addenda99Contested {
        pub fn new(dishonored_return_reason_code: String) -> Addenda99Contested {
            Addenda99Contested {
                dishonored_return_reason_code,
                converters: Converters::new(),
            }
        }
    }

    pub struct Converters {}

    impl Converters {
        pub fn new() -> Converters {
            Converters {}
        }
    }
}
