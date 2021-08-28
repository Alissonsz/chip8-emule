use std::io::Read;
use std::fs::File;

pub struct Emulator {
    memory: [u8; 4096]
}

impl Emulator {
    fn show_memory(&self) {
        for item in self.memory[512..540].iter() {
            println!("{:#02x}", item);
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

    let e = Emulator {
        memory: memory
    };

    e.show_memory();
}