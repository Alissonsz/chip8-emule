pub struct Display {
  screen: [[bool; 64]; 32]
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
    println!("x: {}, y: {}", x, y);
    println!("{:?}", t);

    let mut changed = 0;
    
    for (index, sprite) in t.iter().enumerate() {
      let bin_sprite = format!("{:08b}", sprite);
      for (i, c) in bin_sprite.chars().map(|char| char == '1').enumerate() {
        let cur_value = self.screen[(y + (index as u8)) as usize][(x + (i as u8)) as usize];
        self.screen[(y + (index as u8)) as usize][(x + (i as u8)) as usize] =  cur_value ^ c;
        let updated_value = self.screen[(y + (index as u8)) as usize][(x + (i as u8)) as usize];

        if cur_value != updated_value {
          changed = 1;
        }
        //println!("{}: {}", i, c);
      }
      
    }
    self.print();
    changed
    //println!("{:?}", self.screen);
  }

  pub fn clear(&mut self) {
    self.screen = [[false; 64]; 32];
  }
}