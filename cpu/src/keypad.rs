pub struct KeyPad {
  state: [bool; 16]
}

pub enum Key {
  Key1, Key2, Key3, KeyC,
  Key4, Key5, Key6, KeyD,
  Key7, Key8, Key9, KeyE,
  KeyA, Key0, KeyB, KeyF
}

/*pub const Keys: [u16; 16] = [
  0x0, 0x1, 0x2, 0x3, 
  0x4, 0x5, 0x6, 0x7, 
  0x8, 0x9, 0xA, 0xB, 
  0xC, 0xD, 0xE, 0xF
];*/

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