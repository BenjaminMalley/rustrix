#![feature(rustc_private)]
#![feature(box_syntax)]
#![feature(std_misc)]
#![feature(old_io)]
#![allow(warnings)]
extern crate hyper;
extern crate serialize;
extern crate url;
use std::thread::Thread;
use std::time::Duration;
use serialize::json;
mod client;
mod circuit;
mod error;

#[test]
fn create_client() {
  client::Client::new();
}

