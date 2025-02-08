
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
        if self.reader.as_ref().map_or(false, |r| r.validate_opts().is_none()) {
            return;
        }
        if let Some(ref mut reader) = self.reader {
            reader.set_validation(&opts.clone());
        }
    }
}

impl MoovIoAchReader {
    fn validate_opts(&self) -> &Option<Box<MoovIoAchValidateOpts>> {
        &self.file.header.validate_opts
    }
    
    fn set_validation(&mut self, opts: &MoovIoAchValidateOpts) {
        if self.validate_opts().is_none() {
            return;
        }
        self.file.header.set_validation(&opts.clone());
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

