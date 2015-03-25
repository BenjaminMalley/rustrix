use std::error::Error;
use std::fmt;

pub enum ClientError {
  HostNotFound(String),
  InvalidHostname,
  CircuitClosed,
  RequestTimeout,
}

impl fmt::Display for ClientError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.description())
  }
}

impl Error for ClientError {
  fn description(&self) -> &str {
    match *self {
      ClientError::HostNotFound(_) => "Host Not Found",
      ClientError::InvalidHostname => "Invalid Hostname",
      ClientError::CircuitClosed => "Circuit is Closed",
      ClientError::RequestTimeout => "Request timed out",
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      _ => None,
    }
  }
}