
pub fn decode_set_vx(instruction: u16) -> (u16, u16, u8) {
    let opcode = instruction >> 12;
    let x = (instruction & 0x0F00) >> 8;
    let kk = instruction & 0x00FF ;

    (opcode, x, kk as u8)
}
