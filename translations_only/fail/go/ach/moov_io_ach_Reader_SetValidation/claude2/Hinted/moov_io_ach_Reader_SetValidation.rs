

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
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
    // Fields omitted
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
        let opts = opts.as_ref().unwrap().clone();
        self.file.set_validation(&opts);
    }
}

impl MoovIoAchFile {
    fn set_validation(&mut self, opts: &MoovIoAchValidateOpts) {
        if self.validate_opts.is_none() {
            return;
        }
        
        let opts = opts.clone();
        self.validate_opts = Some(Box::new(opts.clone()));
        self.header.set_validation(&opts);
    }
}

impl MoovIoAchFileHeader {
    fn set_validation(&mut self, opts: &MoovIoAchValidateOpts) {
        if self.validate_opts.is_none() {
            return;
        }
        let opts = opts.clone();
        self.validate_opts = Some(Box::new(opts));
    }
}

