use std::io::{Read, Result, Write};

#[derive(Debug)]
pub struct ReadStats<R> {
    read_bytes: usize,
    read_ops: usize,
    data: R,
}

impl<R: Read> ReadStats<R> {
    pub fn new(data: R) -> ReadStats<R> {
        Self {
            read_bytes: 0,
            read_ops: 0,
            data,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.read_bytes
    }

    pub fn reads(&self) -> usize {
        self.read_ops
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read_bytes = self.data.read(buf)?;
        self.read_bytes += read_bytes;
        self.read_ops += 1;
        Ok(read_bytes)
    }
}

pub struct WriteStats<W> {
    written_bytes: usize,
    written_ops: usize,
    data: W,
}

impl<W: Write> WriteStats<W> {
    pub fn new(data: W) -> WriteStats<W> {
        Self {
            written_bytes: 0,
            written_ops: 0,
            data,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.written_bytes
    }

    pub fn writes(&self) -> usize {
        self.written_ops
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let written_bytes = self.data.write(buf)?;
        self.written_bytes += written_bytes;
        self.written_ops += 1;
        Ok(written_bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
