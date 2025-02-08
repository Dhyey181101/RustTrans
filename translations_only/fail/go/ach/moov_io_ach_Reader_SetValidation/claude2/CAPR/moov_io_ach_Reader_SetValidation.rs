
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
        if opts.is_none() {
            return;
        }
        self.file.set_validation(&opts);
    }
}

impl MoovIoAchFile {
    fn set_validation(&mut self, opts: &Option<Box<MoovIoAchValidateOpts>>) {
        let cloned_opts = opts.as_ref().unwrap().clone();
        let unboxed_opts = *cloned_opts;
        self.validate_opts = Some(Box::new(unboxed_opts.clone()));
        self.header.set_validation(&Some(Box::new(unboxed_opts)));
    }
}

impl MoovIoAchFileHeader {
    fn set_validation(&mut self, opts: &Option<Box<MoovIoAchValidateOpts>>) {
        let unboxed_opts = opts.as_ref().unwrap().clone();
        self.validate_opts = Some(Box::new(*unboxed_opts));
    }
}

