use pbr::ProgressBar;
use std::io::{Write, Result};
use std::io;
pub struct Progress<'a> {
    pb: &'a mut ProgressBar<io::Stdout>,
}

impl<'a> Progress<'a> {
    pub fn new(pb: &mut ProgressBar<io::Stdout>) -> Progress {
        Progress{
            pb: pb,
        }
    }
}

impl<'a> Write for Progress<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
       
        self.pb.add(buf.len() as u64);
        Ok(buf.len() as usize)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}