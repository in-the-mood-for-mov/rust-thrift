pub mod binary_protocol;

pub enum ProtocolExceptionType {
  PeUnknown,
  PeInvalidData,
  PeNegativeSize,
  PeSizeLimit,
  PeBadVersion(~str)
}

condition! {
  pub protocol_exception: super::ProtocolExceptionType -> ();
}

#[deriving(Eq, FromPrimitive)]
pub enum Type {
  Stop = 0x00,
  Void = 0x01,
  Bool = 0x02,
  Byte = 0x03,
  Double = 0x04,
  I16 = 0x06,
  I32 = 0x08,
  I64 = 0x0a,
  String = 0x0b,
  Struct = 0x0c,
  Map = 0x0d,
  Set = 0x0e,
  List = 0x0f
}

#[deriving(Eq, FromPrimitive)]
pub enum MessageType {
  Call = 0x01,
  Reply = 0x02,
  Exception = 0x03,
}

pub trait Protocol {
  fn write_message_begin(
    &mut self,
    name: &str,
    message_type: MessageType,
    sequence_id: i32
  );
  fn write_message_end(&mut self);

  fn write_struct_begin(&mut self, name: &str);
  fn write_struct_end(&mut self);

  fn write_field_begin(&mut self, name: &str, field_type: Type, field_id: i16);
  fn write_field_end(&mut self);
  fn write_field_stop(&mut self);

  fn write_map_begin(&mut self, key_type: Type, value_type: Type, size: i32);
  fn write_map_end(&mut self);

  fn write_list_begin(&mut self, elem_type: Type, size: i32);
  fn write_list_end(&mut self);

  fn write_set_begin(&mut self, elem_type: Type, size: i32);
  fn write_set_end(&mut self);

  fn write_bool(&mut self, value: bool);
  fn write_byte(&mut self, value: i8);
  fn write_i16(&mut self, value: i16);
  fn write_i32(&mut self, value: i32);
  fn write_i64(&mut self, value: i64);
  fn write_double(&mut self, value: f64);
  fn write_string(&mut self, value: &str);
  fn write_binary(&mut self, value: &[u8]);

  fn read_message_begin(&mut self) -> (~str, MessageType, i32);
  fn read_message_end(&mut self);

  fn read_struct_begin(&mut self) -> ~str;
  fn read_struct_end(&mut self);

  fn read_field_begin(&mut self) -> (~str, Type, i16);
  fn read_field_end(&mut self);

  fn read_map_begin(&mut self) -> (Type, Type, i32);
  fn read_map_end(&mut self);

  fn read_list_begin(&mut self) -> (Type, i32);
  fn read_list_end(&mut self);

  fn read_set_begin(&mut self) -> (Type, i32);
  fn read_set_end(&mut self);

  fn read_bool(&mut self) -> bool;
  fn read_byte(&mut self) -> i8;
  fn read_i16(&mut self) -> i16;
  fn read_i32(&mut self) -> i32;
  fn read_i64(&mut self) -> i64;
  fn read_double(&mut self) -> f64;
  fn read_string(&mut self) -> ~str;
  fn read_binary(&mut self) -> ~[u8];
}
