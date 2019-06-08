
use super::node::Progress;
use std::io::{Result, Write};
pub struct ProgressWriter<'a, T, U> {
    file: &'a mut T,
    progress: &'a mut U,
    total: u64,
}

impl<'a, T: Write, U: Progress> ProgressWriter<'a, T, U> {
    pub fn new(file: &'a mut T, progress: &'a mut U, total: u64) -> ProgressWriter<'a, T, U> {
        ProgressWriter {
            file: file,
            progress: progress,
            total,
        }
    }
}

impl<'a, T: Write, U: Progress> Write for ProgressWriter<'a, T, U> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.progress.progress(buf.len() as u64, self.total);
        Ok(self.file.write(buf)?)
    }

    fn flush(&mut self) -> Result<()> {
        self.file.flush()?;
        //self.progress.flush()?;
        Ok(())
    }
}