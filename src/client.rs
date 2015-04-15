#![feature(box_syntax)]
extern crate hyper;
use circuit;
use std::error::Error;
use hyper::client;
use error;
use std::time::duration::Duration;
use std::thread;
use std::sync::mpsc::channel;
use std::old_io::timer::Timer;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Client {
  circuits: Arc<Mutex<HashMap<String, circuit::Circuit>>>,
  timeout: Duration,
}

impl Client {
  pub fn new() -> Client {
    Client{
      circuits: Arc::new(Mutex::new(HashMap::new())),
      timeout: Duration::milliseconds(500),
    }
  }

  pub fn get(&mut self, url: &str) -> Result<hyper::client::response::Response, Box<Error>> {
    let valid_url = try!(client::IntoUrl::into_url(url));
    let cloned_url = valid_url.clone();
    let host = match valid_url.host() {
      Some(host) => host.serialize(),
      None => {
        return Err(box error::ClientError::InvalidHostname);
      }
    };
    let mut circuits = self.circuits.clone();
    let mut c = circuits.lock().unwrap();
    let mut circuit = match c.get_mut(&host) {
      Some(circuit) => circuit,
      None => {
        return Err(box error::ClientError::HostNotFound(host));
      }
    };
    if circuit.is_closed() {
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
          circuit.open();
          let mut cloned_circuits = self.circuits.clone();
          thread::spawn(move || {
            let mut client = client::Client::new();
            let periodic = timer.periodic(Duration::milliseconds(100));
            loop {
              periodic.recv();
              let request = client.get(cloned_url.clone());
              match request.send() {
                Err(_) => continue,
                Ok(_) => {
                  let mut c = cloned_circuits.lock().unwrap();
                  let mut circuit = c.get_mut(&host).unwrap();
                  circuit.close();
                  break;
                },
              }
            }
          });
          return Err(box error::ClientError::RequestTimeout);
        }
      }
    } else {
      return Err(box error::ClientError::CircuitOpen);
    }
  }

  pub fn is_closed(&mut self, host: &str) -> Option<bool> {
    let mut circuits = self.circuits.clone();
    let c = circuits.lock().unwrap();
    match c.get(host) {
      None => None,
      Some(circuit) => Some(circuit.closed)
    }
  }
}

