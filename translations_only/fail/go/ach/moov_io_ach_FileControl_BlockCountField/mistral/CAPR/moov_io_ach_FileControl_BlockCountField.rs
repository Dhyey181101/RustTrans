


use std::fmt;
use std::string::String;

mod moov_io_ach {
    pub struct FileControl {
        block_count: i32,
        converters: Box<Converters>,
    }

    impl FileControl {
        pub fn block_count_field(&self) -> String {
            self.converters.numeric_field(self.block_count, 6)
        }
    }

    pub struct Converters {
        _priv: (),
    }

    impl Converters {
        pub fn new() -> Box<Converters> {
            Box::new(Converters { _priv: () })
        }

        pub fn numeric_field(&self, n: i32, max: u32) -> String {
            let s = n.to_string();
            let l = s.len() as u32;
            if l > max {
                s[(l - max) as usize..].to_string()
            } else {
                let m = max - l;
                let pad = self.get_pad_string(m);
                pad + &s
            }
        }

        pub fn get_pad_string(&self, n: u32) -> String {
            let mut out = vec!['0'; (n + 1) as usize];
            out.reverse();
            out.into_iter().collect()
        }
    }
}

impl fmt::Display for moov_io_ach::FileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.block_count_field())
    }
}


