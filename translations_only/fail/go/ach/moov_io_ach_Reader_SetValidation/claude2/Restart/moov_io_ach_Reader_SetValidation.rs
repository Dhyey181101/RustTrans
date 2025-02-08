
use std::boxed::Box;

#[derive(Clone)]
struct MoovIoAchValidateOpts {
    skip_all: bool,
    // Other fields omitted
}

struct MoovIoAchFile {
    header: MoovIoAchFileHeader,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

struct MoovIoAchFileHeader {
    // Fields omitted
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

impl MoovIoAchFile {
    fn set_validation(&mut self, opts: &Option<Box<MoovIoAchValidateOpts>>) {
        if let Some(o) = opts {
            self.validate_opts = Some(o.clone());
            self.header.set_validation(opts);
        }
    }
}

impl MoovIoAchFileHeader {
    fn set_validation(&mut self, opts: &Option<Box<MoovIoAchValidateOpts>>) {
        if let Some(o) = opts {
            self.validate_opts = Some(o.clone());
        }
    }
}

