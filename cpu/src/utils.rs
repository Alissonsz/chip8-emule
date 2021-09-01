pub fn combine_3nibbles(x: u8, y: u8, z: u8) -> u16 {
  let mut combined = (x as u16) << 8;
  //println!("Combined: {:#02x}", combined);
  combined |= (y as u16) << 4;
  //println!("Combined: {:#02x}", combined);
  combined |= z as u16;
  //println!("Combined: {:#02x}", combined);

  combined
}

pub fn combine_2nibbles(n: u8, nn: u8) -> u8 {
  let combined = (n << 4) | nn;
  //println!("Combined: {:#02x}", combined);
  combined as u8
}

pub fn combine_2bytes(x: u8, y: u8) -> u16 {
  ((x as u16) << 8) | (y as u16)
}