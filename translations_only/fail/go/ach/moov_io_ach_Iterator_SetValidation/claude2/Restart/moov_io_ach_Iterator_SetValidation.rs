
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
    validate_opts: Option<MoovIoAchValidateOpts>,
    // other fields omitted
}

struct MoovIoAchFileHeader {
    validate_opts: Option<MoovIoAchValidateOpts>,
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
        self.file.set_validation(opts.clone());
    }
}

impl MoovIoAchFile {
    fn set_validation(&mut self, opts: MoovIoAchValidateOpts) {
        if opts.is_none {
            return;
        }
        self.validate_opts = Some(opts.clone());
        self.header.set_validation(opts);
    }
}

impl MoovIoAchFileHeader {
    fn set_validation(&mut self, opts: MoovIoAchValidateOpts) {
        if opts.is_none {
            return;
        }
        self.validate_opts = Some(opts);
    }
}

