
use std::boxed::Box;

#[derive(Clone)]
struct MoovIoAchValidateOpts {
    skip_all: bool,
    // Other fields omitted
}

struct MoovIoAchFile {
    header: Box<MoovIoAchFileHeader>,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

struct MoovIoAchFileHeader {
    // Fields omitted
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

struct MoovIoAchReader {
    file: MoovIoAchFile,
    // Other fields omitted
}

impl MoovIoAchReader {
    fn set_validation(&mut self, opts: &Option<Box<MoovIoAchValidateOpts>>) {
        if self.file.validate_opts.is_none() || opts.is_none() {
            return;
        }
        self.file.set_validation(opts);
    }
}

impl MoovIoAchFile {
    fn set_validation(&mut self, opts: &Option<Box<MoovIoAchValidateOpts>>) {
        if let Some(opts) = opts {
            if let Some(ref mut validate_opts) = self.validate_opts {
                *validate_opts = opts.clone();
                self.header.set_validation(&opts);
            }
        }
    }
}

impl MoovIoAchFileHeader {
    fn set_validation(&mut self, opts: &Box<MoovIoAchValidateOpts>) {
        if let Some(ref mut validate_opts) = self.validate_opts {
            *validate_opts = opts.clone();
        }
    }
}

