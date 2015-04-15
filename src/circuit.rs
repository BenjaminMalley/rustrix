pub struct Circuit {
  pub closed: bool,
}

impl Circuit {
  pub fn new() -> Circuit {
    Circuit{
      closed: true,
    }
  }

  pub fn close(&mut self) {
    self.closed = true;
  }

  pub fn open(&mut self) {
    self.closed = false;
  }

  pub fn is_closed(&self) -> bool {
    self.closed
  }
}

