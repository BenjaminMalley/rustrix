pub struct Circuit {
  pub open: bool,
}

impl Circuit {
  pub fn new() -> Circuit {
    Circuit{
      open: true,
    }
  }

  pub fn close(&mut self) {
    self.open = false;
  }

  pub fn open(&mut self) {
    self.open = true;
  }

  pub fn is_open(&self) -> bool {
    self.open
  }
}

