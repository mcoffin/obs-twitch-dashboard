use open;

use std::io::Result;
use std::process;

pub trait Open {
    fn open(&self) -> Result<process::ExitStatus>;
}

impl<T> Open for T where T: AsRef<str> {
    fn open(&self) -> Result<process::ExitStatus> {
        open::that(self.as_ref())
    }
}
