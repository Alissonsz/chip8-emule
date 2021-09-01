
pub fn decode_set_vx(instruction: u16) -> (u16, u16, u8) {
  let opcode = instruction >> 12;
  let x = (instruction & 0x0F00) >> 8;
  let kk = instruction & 0x00FF ;

  (opcode, x, kk as u8)
}

pub fn transform_array(instruction: u16) -> [u8; 4] {
  let opcode = instruction >> 12;
  let x = (instruction & 0x0F00) >> 8;
  let k1 = (instruction & 0x00F0) >> 4;
  let k2 = instruction & 0x000F;

  [opcode as u8, x as u8, k1 as u8, k2 as u8]
}
