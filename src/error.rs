use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum ClientError {
  HostNotFound(String),
  InvalidHostname,
  CircuitOpen,
  RequestTimeout,
  HttpClientError,
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
      ClientError::CircuitOpen => "Circuit is Open",
      ClientError::RequestTimeout => "Request timed out",
      ClientError::HttpClientError => "Http Client Error",
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      _ => None,
    }
  }
}
