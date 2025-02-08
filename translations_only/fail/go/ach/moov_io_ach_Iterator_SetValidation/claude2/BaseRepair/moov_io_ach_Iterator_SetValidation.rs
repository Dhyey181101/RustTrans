

use std::boxed::Box;

#[derive(Clone)]
struct MoovIoAchValidateOpts {
    is_none: bool,
    // fields omitted for brevity
}

struct MoovIoAchIterator {
    reader: Option<Box<MoovIoAchReader>>,  
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
}

impl MoovIoAchIterator {
    fn set_validation(&mut self, opts: &MoovIoAchValidateOpts) {
        if let Some(ref mut reader) = self.reader {
            reader.set_validation(&opts.clone());
        }
    }
}

impl MoovIoAchReader {
    fn set_validation(&mut self, opts: &MoovIoAchValidateOpts) {
        if opts.is_none {
            return;
        }
        self.file.header.set_validation(&opts.clone());
    }
}

impl MoovIoAchFileHeader {
    fn set_validation(&mut self, opts: &MoovIoAchValidateOpts) {
        if opts.is_none {
            return;
        }
        self.validate_opts = Some(Box::new(opts.clone()));
    }
}

