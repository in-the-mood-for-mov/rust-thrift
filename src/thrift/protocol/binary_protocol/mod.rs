use std;
use protocol;
use protocol::{ MessageType, Protocol, Type };
use transport::Transport;

static BINARY_PROTOCOL_VERSION_1: u16 = 0x8001;

pub struct BinaryProtocol {
  transport: ~Transport
}

impl BinaryProtocol {
  fn new(transport: ~Transport) -> BinaryProtocol {
    BinaryProtocol { transport: transport }
  }

  fn write_type(&mut self, type_: Type) {
    self.write_byte(type_ as i8);
  }

  fn read_type(&mut self) -> Type {
    let raw = self.read_byte();
    match FromPrimitive::from_i8(raw) {
      Some(type_) => type_,
      None => fail!("unknown type {}", raw),
    }
  }
}

impl Protocol for BinaryProtocol {
  fn write_message_begin(
    &mut self,
    name: &str,
    message_type: MessageType,
    sequence_id: i32
  ) {
    let version = BINARY_PROTOCOL_VERSION_1 as i32 | message_type as i32;
    self.write_i32(version);
    self.write_string(name);
    self.write_i32(sequence_id);
  }

  fn write_message_end(&mut self) { }

  fn write_struct_begin(&mut self, _name: &str) { }

  fn write_struct_end(&mut self) { }

  fn write_field_begin(
    &mut self,
    _name: &str,
    field_type: Type,
    field_id: i16
  ) {
    self.write_type(field_type);
    self.write_i16(field_id);
  }

  fn write_field_end(&mut self) { }

  fn write_field_stop(&mut self) {
    self.write_byte(protocol::TStop as i8);
  }

  fn write_map_begin(&mut self, key_type: Type, value_type: Type, size: i32) {
    self.write_type(key_type);
    self.write_type(value_type);
    self.write_i32(size);
  }

  fn write_map_end(&mut self) { }

  fn write_list_begin(&mut self, elem_type: Type, size: i32) {
    self.write_type(elem_type);
    self.write_i32(size);
  }

  fn write_list_end(&mut self) { }

  fn write_set_begin(&mut self, elem_type: Type, size: i32) {
    self.write_type(elem_type);
    self.write_i32(size);
  }

  fn write_set_end(&mut self) { }

  fn write_bool(&mut self, value: bool) {
    self.write_byte(value as i8);
  }

  fn write_byte(&mut self, value: i8) {
    match self.transport.write_i8(value) {
      Ok(_) => (),
      Err(e) => fail!(e),
    }
  }

  fn write_i16(&mut self, value: i16) {
    match self.transport.write_be_i16(value) {
      Ok(_) => (),
      Err(e) => fail!(e),
    }
  }

  fn write_i32(&mut self, value: i32) {
    match self.transport.write_be_i32(value) {
      Ok(_) => (),
      Err(e) => fail!(e),
    }
  }

  fn write_i64(&mut self, value: i64) {
    match self.transport.write_be_i64(value) {
      Ok(_) => (),
      Err(e) => fail!(e),
    }
  }

  fn write_double(&mut self, value: f64) {
    match self.transport.write_be_f64(value) {
      Ok(_) => (),
      Err(e) => fail!(e),
    }
  }

  fn write_string(&mut self, value: &str) {
    self.write_binary(value.as_bytes());
  }

  fn write_binary(&mut self, value: &[u8]) {
    self.write_i32(value.len() as i32);
    match self.transport.write(value) {
      Ok(_) => (),
      Err(e) => fail!(e),
    }
  }

  fn read_message_begin(&mut self) -> (~str, MessageType, i32) {
    let header = self.read_i32();
    let version = (header >> 16) as u16;
    if version != BINARY_PROTOCOL_VERSION_1 {
      fail!("unknown protocol version: {:x}", version);
    };
    let name = self.read_string();
    let raw_type = header & 0xff;
    let message_type = match FromPrimitive::from_i32(raw_type) {
      Some(t) => t,
      None => fail!("unknown message type {:x}", raw_type),
    };
    let sequence_id = self.read_i32();
    (name, message_type, sequence_id)
  }

  fn read_message_end(&mut self) { }

  fn read_struct_begin(&mut self) -> ~str { ~"" }

  fn read_struct_end(&mut self) { }

  fn read_field_begin(&mut self) -> (~str, Type, i16) {
    let field_type = self.read_type();
    let field_id = match field_type {
      protocol::TStop => 0,
      _ => self.read_i16(),
    };
    (~"", field_type, field_id)
  }

  fn read_field_end(&mut self) { }

  fn read_map_begin(&mut self) -> (Type, Type, i32) {
    let key_type = self.read_type();
    let value_type = self.read_type();
    let size = self.read_i32();
    (key_type, value_type, size)
  }

  fn read_map_end(&mut self) { }

  fn read_list_begin(&mut self) -> (Type, i32) {
    let elem_type = self.read_type();
    let size = self.read_i32();
    (elem_type, size)
  }

  fn read_list_end(&mut self) { }

  fn read_set_begin(&mut self) -> (Type, i32) {
    let elem_type = self.read_type();
    let size = self.read_i32();
    (elem_type, size)
  }

  fn read_set_end(&mut self) { }

  fn read_bool(&mut self) -> bool {
    match self.read_byte() {
      0 => false,
      _ => true,
    }
  }

  fn read_byte(&mut self) -> i8 {
    self.transport.read_i8().unwrap()
  }

  fn read_i16(&mut self) -> i16 {
    self.transport.read_be_i16().unwrap()
  }

  fn read_i32(&mut self) -> i32 {
    self.transport.read_be_i32().unwrap()
  }

  fn read_i64(&mut self) -> i64 {
    self.transport.read_be_i64().unwrap()
  }

  fn read_double(&mut self) -> f64 {
    self.transport.read_be_f64().unwrap()
  }

  fn read_string(&mut self) -> ~str {
    std::str::from_utf8_owned(self.read_binary()).unwrap()
  }

  fn read_binary(&mut self) -> ~[u8] {
    let len = self.read_i32() as uint;
    self.transport.read_exact(len).unwrap()
  }
}

#[cfg(test)]
pub mod test;
