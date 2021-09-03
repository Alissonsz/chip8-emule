pub struct Display {
  pub screen: [[bool; 64]; 32]
}

impl Display {
  pub fn new() -> Display {
    Display {
      screen: [[false; 64]; 32]
    }
  }

  fn print(&self) {
    for (i, v) in self.screen.iter().enumerate() {
      for (k, t) in self.screen[i].iter().enumerate() {
        let number = if *t {
          "1"
        } else {
          " "
        };

        print!("{} ", number);
      }
      println!();
    }
  }

  pub fn draw(&mut self, x: u8, y: u8, t: Vec<u8>) -> u8 {
    let mut changed = 0;
    
    for (index, sprite) in t.iter().enumerate() {
      let bin_sprite = format!("{:08b}", sprite);
      if (y + (index as u8)) >= 32 {
        break;
      }
      for (i, c) in bin_sprite.chars().map(|char| char == '1').enumerate() {
        if (x + (i as u8)) >= 64 {
          break;
        }
        let cur_value = self.screen[(y + (index as u8)) as usize][(x + (i as u8)) as usize];
        self.screen[(y + (index as u8)) as usize][(x + (i as u8)) as usize] =  cur_value ^ c;
        let updated_value = self.screen[(y + (index as u8)) as usize][(x + (i as u8)) as usize];

        if (cur_value == true) && (updated_value == false) {
          changed = 1;
        }
      }
    }
 
    changed
  }

  pub fn clear(&mut self) {
    self.screen = [[false; 64]; 32];
  }

  pub fn get_display_as_u8_texture(&self) -> Vec<u8> {
    let mut tex = Vec::new();
    
    for (i, v) in self.screen.iter().enumerate() {
      for (j, t) in v.iter().enumerate() {
        if *t {
          tex.push(255);
        } else {
          tex.push(0);
        }
      }
    }

    tex
  }
}