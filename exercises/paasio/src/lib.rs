use std::io::{Read, Result, Write};

pub struct ReadStats<R>{
    inner:R,
    bytes:usize,
    reads:usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        ReadStats {
            inner:_wrapped,
            bytes:0,
            reads:0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let through = self.inner.read(buf)?;
        self.bytes += through;
        self.reads += 1;
        Ok(through)
    }
}

pub struct WriteStats<W>{
    inner:W,
    bytes:usize,
    writes:usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        WriteStats{
            inner:_wrapped,
            bytes:0,
            writes:0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let through = self.inner.write(buf)?;
        self.bytes += through;
        self.writes += 1;
        Ok(through)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
