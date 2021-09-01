use std::io::Read;
use std::fs::File;

mod instructions;

pub struct Emulator {
    memory: [u8; 4096],
    registers: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16]
}

impl Emulator {
    fn show_memory(&self) {
        for item in self.memory[512..540].iter() {
            println!("{:#02x}", item);
        }
    }

    fn interpret_instruction(&mut self, instruction: u16) {
        println!("{:#02x}", instruction & 0xF000);
        match instruction >> 12 {
            0x1 => { println!("É UM JUMP CARA"); self.pc = instruction & 0x0FFF },
            0x6 => { 
                println!("É UM SET CARA"); 
                let (_, reg, value) = instructions::decode_set_vx(instruction);
                self.registers[reg as usize] = value; 
            }
            _ => println!("É JUMP NÃO CARA")
        }
    }
}

pub fn run(filename: &String) {
    let mut f = File::open(&filename).expect("Could not open file");
    let metadata = std::fs::metadata(&filename).expect("Unable to red metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    let mut memory = [0; 4096];
    
    for (index, instruction) in buffer.iter().enumerate() {
        memory[index + 512] = *instruction;
    }

    let mut e = Emulator {
        memory: memory,
        registers: [0; 16],
        i: 0,
        pc: 512,
        sp: 0,
        stack: [0; 16]
    };

    /*println!("{}", e.pc);
    e.interpret_instruction(((e.memory[e.pc as usize] as u16) << 8) | (e.memory[(e.pc + 1) as usize] as u16));
    println!("{}", e.pc);
    e.interpret_instruction(((e.memory[e.pc as usize] as u16) << 8) | (e.memory[(e.pc + 1) as usize] as u16));
    println!("{:#?}", e.registers);*/
}