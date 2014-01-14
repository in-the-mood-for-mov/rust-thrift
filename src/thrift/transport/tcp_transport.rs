use std::io::net;

impl super::Transport for net::tcp::TcpStream {
  fn flush(&mut self) { }
}
