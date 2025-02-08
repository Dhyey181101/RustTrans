

use std::boxed::Box;
use std::option::Option;

#[derive(Clone)]
struct MoovIoAchIterator {
    reader: Option<Box<MoovIoAchReader>>,
}

#[derive(Clone)]
struct MoovIoAchValidateOpts {
    // ...
}

#[derive(Clone)]
struct MoovIoAchReader {
    file: MoovIoAchFile,
}

#[derive(Clone)]
struct MoovIoAchFile {
    header: MoovIoAchFileHeader,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

#[derive(Clone)]
struct MoovIoAchFileHeader {
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
    // ...
}

impl MoovIoAchIterator {
    fn set_validation(&mut self, opts: &Option<Box<MoovIoAchValidateOpts>>) {
        if let Some(ref mut reader) = self.reader {
            reader.set_validation(opts);
        }
    }
}

impl MoovIoAchReader {
    fn set_validation(&mut self, opts: &Option<Box<MoovIoAchValidateOpts>>) {
        if self.file.validate_opts.is_some() || opts.is_some() {
            self.file.validate_opts = opts.clone();
            self.file.header.set_validation(&self.file.validate_opts);
        }
    }
}

impl MoovIoAchFileHeader {
    fn set_validation(&mut self, opts: &Option<Box<MoovIoAchValidateOpts>>) {
        self.validate_opts = opts.clone();
    }
}

