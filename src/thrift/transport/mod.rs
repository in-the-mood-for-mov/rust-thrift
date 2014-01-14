use std::io::{Writer, Reader};

pub mod tcp_transport;

pub trait Transport : Writer + Reader {
  fn flush(&mut self);
}
