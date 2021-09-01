use cpu;

fn main() {
  let mut emulator = cpu::new(&String::from("IBM Logo.ch8"));

  emulator.run();
}