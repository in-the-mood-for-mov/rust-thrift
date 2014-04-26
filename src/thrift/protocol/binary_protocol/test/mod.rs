use std::io::{ IoError, MemReader, MemWriter, Reader, Writer };
use protocol::Protocol;
use transport::Transport;
use super::BinaryProtocol;

pub struct FakeTransport {
  reader: MemReader,
  writer: MemWriter,
}

impl FakeTransport {
  fn new(buf: ~[u8]) -> FakeTransport {
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

#[test]
pub fn test_read_byte() {
  let transport = ~FakeTransport::new(~[0xa4, 0x27]);
  let  protocol: &mut Protocol = &mut BinaryProtocol::new(transport);
  assert_eq!(protocol.read_byte(), -0x5c);
  assert_eq!(protocol.read_byte(), 0x27);
}

#[test]
pub fn test_read_i16() {
  let transport = ~FakeTransport::new(~[0xf2, 0xf8, 0xa1, 0x40]);
  let protocol = &mut BinaryProtocol::new(transport);
  assert_eq!(protocol.read_i16(), -0x0d08);
  assert_eq!(protocol.read_i16(), -0x5ec0);
}

#[test]
pub fn test_read_i32() {
  let transport = ~FakeTransport::new(~[0x27, 0xd0, 0x39, 0x49, 0xe5, 0xd8, 0xfe, 0x8b]);
  let protocol = &mut BinaryProtocol::new(transport);
  assert_eq!(protocol.read_i32(), 0x27d03949);
  assert_eq!(protocol.read_i32(), -0x1a270175);
}

#[test]
pub fn test_read_i64() {
  let transport = ~FakeTransport::new(~[
    0x27, 0xd0, 0x39, 0x49, 0xe5, 0xd8, 0xfe, 0x8b,
    0xa7, 0x2e, 0x82, 0xea, 0xd1, 0x28, 0x0b, 0xe2,
  ]);
  let protocol = &mut BinaryProtocol::new(transport);
  assert_eq!(protocol.read_i64(), 0x27d03949e5d8fe8b);
  assert_eq!(protocol.read_i64(), -0x58d17d152ed7f41e);
}

#[test]
pub fn test_read_double() {
  let transport = ~FakeTransport::new(~[
    0x40, 0xa9, 0x5e, 0xaf, 0x39, 0x4b, 0x7b, 0x29,
    0xbf, 0xe9, 0x3a, 0xe4, 0x21, 0xd3, 0x0e, 0x85,
  ]);
  let protocol = &mut BinaryProtocol::new(transport);
  assert_eq!(protocol.read_double(), 3247.342234);
  assert_eq!(protocol.read_double(), -0.78843886);
}

#[test]
pub fn test_read_string() {
  let transport = ~FakeTransport::new(~[
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x04, 0x41, 0x73, 0x64, 0x66,
    0x00, 0x00, 0x00, 0x0d, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21,
  ]);
  let protocol = &mut BinaryProtocol::new(transport);
  assert_eq!(protocol.read_string(), ~"");
  assert_eq!(protocol.read_string(), ~"Asdf");
  assert_eq!(protocol.read_string(), ~"Hello, World!");
}
