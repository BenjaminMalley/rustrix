#![feature(rustc_private)]
#![feature(box_syntax)]
extern crate hyper;
extern crate serialize;
extern crate url;
use std::thread::Thread;
use std::old_io::timer;
use std::time::Duration;
use std::sync::mpsc::channel;
use serialize::json;
mod client;
mod circuit;
mod error;

#[test]
fn create_client() {
  client::Client::new();
}

