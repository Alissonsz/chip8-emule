pub struct Display {
  screen: [[u8; 32]; 64]
}

impl Display {
  pub fn new() -> Display {
    Display {
      screen: [[0; 32]; 64]
    }
  }

  pub fn draw(&self, x: u8, y: u8, t: Vec<u8>) {
    println!("x: {}, y: {}", x, y);
    println!("{:?}", t);
    
  
    println!("{:?}", self.screen);
  }
}