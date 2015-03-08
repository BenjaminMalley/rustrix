#![feature(rustc_private)]
#![feature(box_syntax)]
extern crate hyper;
extern crate serialize;
use std::thread::Thread;
use std::old_io::timer;
use std::time::Duration;
use std::sync::mpsc::channel;
use hyper::client::Client;
use serialize::json;
use std::error::Error;

#[derive(Decodable, Encodable, Debug)]
struct Module {
  name: String,
  latestVersion: String,
  org: String,
  owner: String,
}

fn get(url: &str) -> Result<hyper::client::Response, Box<Error>> {
  let mut client = Client::new();
  let the_url = hyper::client::IntoUrl::into_url(url);
  match client.get(url).send() {
    Err(e) => Err(box e),
    Ok(x) => Ok(x),
  }
}

