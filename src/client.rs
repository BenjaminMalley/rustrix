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
use std::old_io::timer;

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

  pub fn get(&mut self, url: &str) -> Result<client::Response, Box<Error>> {
    let mut client = client::Client::new();
    let valid_url = try!(client::IntoUrl::into_url(url));
    let host = match valid_url.host() {
      Some(host) => host.serialize(),
      None => {
        return Err(box error::ClientError::InvalidHostname);
      }
    };
    let circuit = match self.circuits.get(&host) {
      Some(circuit) => circuit,
      None => {
        return Err(box error::ClientError::HostNotFound(host));
      }
    };
    if circuit.is_open() {
      let (tx, rx) = channel();
      let tx2 = tx.clone();
      let request = client.get(valid_url);
      thread::spawn(|| {
        let res = match request.send() {
          Err(e) => Err(box e),
          Ok(x) => Ok(x),
        };
        tx.send(res);
      });

      thread::spawn(move || {
        timer::sleep(self.timeout);
        circuit.close();
        tx2.send(Err(box Client::RequestTimeout));
      });

      match rx.recv() {
        Ok(x) => x,
        Err(e) => Err(box e),
      }

    } else {
      Err(box error::ClientError::CircuitClosed);
    }
  }

  pub fn is_open(&mut self, host: &str) -> Option<bool> {
    match self.circuits.get(host) {
      None => None,
      Some(circuit) => Some(circuit.open)
    }
  }
}

