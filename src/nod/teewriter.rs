use std::io::{Write, Result};

pub struct TeeWriter<T, U> {
    file: T,
    progress: U,
}

impl<'a, T: Write, U: Write> TeeWriter<T, U> {
    pub fn new(file: T, progress: U) -> TeeWriter<T, U> {
        TeeWriter {
            file: file,
            progress: progress,
        }
    }
}

impl<T: Write, U: Write> Write for TeeWriter<T, U> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        println!("write");
        //let size = self.file.write(buf)?;
        self.progress.write(buf)?;
        //Ok(size as usize)
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.file.flush()?;
        self.progress.flush()?;
        Ok(())
    }
}