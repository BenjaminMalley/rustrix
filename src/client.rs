#![feature(box_syntax)]
extern crate hyper;
use std::collections::BTreeMap;
use circuit;
use std::error::Error;
use hyper::client;
use error;
use std::time::duration::Duration;
use std::thread;
use std::sync::mpsc::channel;
use std::old_io::timer::Timer;

pub struct Client {
  circuits: BTreeMap<String, circuit::Circuit>,
  timeout: Duration,
}

impl Client {
  pub fn new() -> Client {
    Client{
      circuits: BTreeMap::new(),
      timeout: Duration::milliseconds(500),
    }
  }

  pub fn get(&mut self, url: &str) -> Result<hyper::client::response::Response, Box<Error>> {
    let valid_url = try!(client::IntoUrl::into_url(url));
    let host = match valid_url.host() {
      Some(host) => host.serialize(),
      None => {
        return Err(box error::ClientError::InvalidHostname);
      }
    };
    let mut circuit = match self.circuits.get_mut(&host) {
      Some(circuit) => circuit,
      None => {
        return Err(box error::ClientError::HostNotFound(host));
      }
    };
    if circuit.is_open() {
      let (tx, rx) = channel();
      thread::spawn(move || {
        let mut client = client::Client::new();
        let request = client.get(valid_url);
        tx.send(match request.send() {
          Err(e) => Err(box e),
          Ok(x) => Ok(x),
        });
      });
      let mut timer = Timer::new().unwrap();
      let timeout = timer.oneshot(self.timeout);

      select! {
        msg = rx.recv() => {
          match msg {
            Ok(resp) => return match resp {
              Ok(r) => Ok(r),
              Err(e) => Err(box error::ClientError::HttpClientError),
            },
            Err(e) => return Err(box e),
          };
        },
        _ = timeout.recv() => {
          circuit.close();
          return Err(box error::ClientError::RequestTimeout);
        }
      }
    } else {
      return Err(box error::ClientError::CircuitClosed);
    }
  }

  pub fn is_open(&mut self, host: &str) -> Option<bool> {
    match self.circuits.get(host) {
      None => None,
      Some(circuit) => Some(circuit.open)
    }
  }
}

