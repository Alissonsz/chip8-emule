pub struct KeyPad {
  state: [bool; 16]
}

pub enum Key {
  Key0, Key1, Key2, Key3,
  Key4, Key5, Key6, Key7,
  Key8, Key9, KeyA, KeyB,
  KeyC, KeyD, KeyE, KeyF
}

impl KeyPad {
  pub fn set_pressed(&mut self, key: Key) {
    self.state[key as usize] = true;
  }

  pub fn set_unpressed(&mut self, key: Key) {
    self.state[key as usize] = false;
  }

  pub fn get_key(&self, key: u8) -> bool {
    self.state[key as usize]
  }

  pub fn new() -> KeyPad {
    KeyPad {
      state: [false; 16]
    }
  }
}