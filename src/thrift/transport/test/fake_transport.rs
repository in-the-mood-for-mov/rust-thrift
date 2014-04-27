use std::io::{ IoError, MemReader, MemWriter, Reader, Writer };
use transport::Transport;

pub struct FakeTransport {
  reader: MemReader,
  writer: MemWriter,
}

impl FakeTransport {
  pub fn new(buf: ~[u8]) -> FakeTransport {
    FakeTransport {
      reader: MemReader::new(buf),
      writer: MemWriter::new(),
    }
  }
}

impl Writer for FakeTransport {
  fn write(&mut self, buf: &[u8]) -> Result<(), IoError> {
    self.writer.write(buf)
  }

  fn flush(&mut self) -> Result<(), IoError> {
    self.writer.flush()
  }
}

impl Reader for FakeTransport {
  fn read(&mut self, buf: &mut [u8]) -> Result<uint, IoError> {
    self.reader.read(buf)
  }
}

impl Transport for FakeTransport { }
