
use std::boxed::Box;

#[derive(Clone)]
struct MoovIoAchValidateOpts {
    // fields omitted for brevity
}

struct MoovIoAchReader {
    file: MoovIoAchFile,
    // other fields omitted
}

struct MoovIoAchFile {
    header: MoovIoAchFileHeader,
    // other fields omitted
}

struct MoovIoAchFileHeader {
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
    // other fields omitted
}

struct MoovIoAchIterator {
    reader: Option<Box<MoovIoAchReader>>,
}

impl MoovIoAchIterator {
    fn set_validation(&mut self, opts: &MoovIoAchValidateOpts) {
        if self.reader.is_some() {
            self.reader.as_mut().unwrap().set_validation(&opts);
        }
    }
}

impl MoovIoAchReader {
    fn set_validation(&mut self, opts: &MoovIoAchValidateOpts) {
        if self.file.header.validate_opts.is_none() {
            return;
        }
        self.file.header.set_validation(&opts);
    }
}

impl MoovIoAchFileHeader {
    fn set_validation(&mut self, opts: &MoovIoAchValidateOpts) {
        if self.validate_opts.is_none() {
            return;
        }
        self.validate_opts = Some(Box::new(opts.clone()));
    }
}

