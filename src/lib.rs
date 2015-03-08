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


/*
fn run<A: Send + 'static, F: Fn() -> A + Send + 'static>(f: F) -> Result<A, String> {
  let (tx1, rx1) = channel();
  Thread::spawn(move|| {
    tx1.send(f());
  });

  let (tx2, rx2) = channel();
  Thread::spawn(move|| {
    timer::sleep(Duration::milliseconds(50));
    tx2.send("timeout".to_string());
  });

  select! (
    result = rx1.recv() => match result {
      Ok(x) => Ok(x),
      Err(_) => Err("Bad result".to_string()),
    },
    _ = rx2.recv() => Err("Timed out".to_string())
  )
}

#[test]
fn do_something() {
  fn return1 () -> i32 { 1 }
  println!("{}", run(return1).unwrap());
}
*/

fn bind<'a, R, T, F: Fn(T) -> Result<R, &'a Error>>(m: Result<T, &'a Error>, f: F) -> Result<R, &'a Error> {
  match m {
    Ok(r) => f(r),
    Err(e) => Err(e),
  }
}

fn get(url: &str) -> Result<hyper::client::Response, Box<Error>> {
  let mut client = Client::new();
  let the_url = hyper::client::IntoUrl::into_url(url);
  match client.get(url).send() {
    Err(e) => Err(box e),
    Ok(x) => Ok(x),
  }
}

/*
#[test]
fn test_hyper() {
  let mut client = Client::new();
  let mut res = client.get("http://127.0.0.1:9000/modules/rxjava").send().unwrap();
  assert_eq!(res.status, hyper::Ok);
  let module: Module = bind(res.read_to_string(), |body| json::decode(&body)).unwrap_or(Module {
    name: "rxjava".to_string(),
    latestVersion: "1.0.0".to_string(),
    org: "EngTools".to_string(),
    owner: "Ben".to_string(),
  });
  println!("{:?}", module);
}
*/
