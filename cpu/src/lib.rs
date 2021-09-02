use std::io::Read;
use std::fs::File;

use display::Display;

use crate::fontset::FONTSET;

mod instructions;
mod utils;
mod display;
mod fontset;

pub struct Emulator {
  memory: [u8; 4096],
  registers: [u8; 16],
  i: u16,
  pc: u16,
  sp: u8,
  stack: [u16; 16],
  display: Display
}

impl Emulator {
  pub fn run(&mut self) {
    
    let mut counter = 0;

    while counter < 20 {
      println!("{}", self.pc);
      self.interpret_instruction(utils::combine_2bytes(self.memory[self.pc as usize], self.memory[(self.pc + 1) as usize]));
      counter += 1;
    }
  }
    
  

  fn show_memory(&self) {
    for item in self.memory[512..540].iter() {
      println!("{:#02x}", item);
    }
  }

  fn interpret_instruction(&mut self, instruction: u16) {
    let dec_instruction = instructions::transform_array(instruction);

    self.pc += 2;

    match dec_instruction {
      [0x0, 0x0, 0xE, 0x0] => { self.display.clear() },
      [0x1, x, y, z] => {self.pc = utils::combine_3nibbles(x, y, z); },
      [0x6, x, n, nn] => { 
          self.registers[x as usize] = utils::combine_2nibbles(n, nn); 
      },
      [0x7, x, n, nn] => { 
        self.registers[x as usize] += utils::combine_2nibbles(n, nn);
      },
      [0xA, n, nn, nnn] => {
        self.i = utils::combine_3nibbles(n, nn, nnn);
      },
      [0xD, vx, vy, n] => {
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        let sprite_start = self.i as usize;
        let sprite_end = (self.i + (n as u16)) as usize;

        let mut sprites = Vec::new();
        for item in self.memory[sprite_start..sprite_end].iter() {
          sprites.push(*item);
        }

        self.registers[0xF] = self.display.draw(x, y, sprites);
      },
      _ => println!("TEM AINDA NÃO MAS SE PÁ VAI TER")
    }
  }
}

pub fn new(filename: &String) -> Emulator {
  let mut f = File::open(&filename).expect("Could not open file");
  let metadata = std::fs::metadata(&filename).expect("Unable to red metadata");
  let mut buffer = vec![0; metadata.len() as usize];
  f.read(&mut buffer).expect("buffer overflow");

  let mut memory = [0; 4096];
  
  //load program in memory
  for (index, instruction) in buffer.iter().enumerate() {
    memory[index + 512] = *instruction;
  }

  //load fontset in memory
  for (index, value) in FONTSET.iter().enumerate() {
    memory[index + 0x50] = *value;
  }

  let mut e = Emulator {
    memory: memory,
    registers: [0; 16],
    i: 0,
    pc: 512,
    sp: 0,
    stack: [0; 16],
    display: Display::new()
  };

  e
}