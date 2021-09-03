use renderer;
use renderer::Renderer;

use cpu;
use cpu::keypad;

extern crate glfw;

use self::glfw::{Action, Key};

use std::sync::mpsc::Receiver;

fn main() {
  let (renderer, events, mut window, mut glfw) = Renderer::new();
  let mut vertices: Vec<f32> = vec![
    -2.0, 1.20, 0.0, 0.0, 0.0,
    2.0, 1.20, 0.0, 1.0, 0.0,
    -2.0, -1.20, 0.0, 0.0, 1.0,
    2.0, 1.20, 0.0, 1.0, 0.0,
    2.0, -1.20, 0.0, 1.0, 1.0,
    -2.0, -1.20, 0.0, 0.0, 1.0
  ];

  let mut emulator = cpu::new(&String::from("tetris.ch8"));
  let mut last_time = glfw.get_time();
  let mut framecount = 0;
  while !window.should_close() {
    let time = glfw.get_time();
    framecount += 1;

    if time - last_time >= 1.0 {
      //println!("{}", framecount);
      framecount = 0;
      last_time = time;
    }
    
    glfw.poll_events();
    process_events(&mut window, &events, &mut emulator);
    emulator.run_cicle();
    renderer.set_texture(&mut emulator.display.get_display_as_u8_texture());
    renderer.draw_with_texure(&mut vertices);
    renderer::swap_buffer(&mut window);
  };
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, emulator: &mut cpu::Emulator) {
  for (_, event) in glfw::flush_messages(events) {
      match event {
          glfw::WindowEvent::FramebufferSize(width, height) => {
            renderer::update_window_buffer(width, height);
          }
          glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
          glfw::WindowEvent::Key(Key::Num1, _, Action::Press, _) => {
            emulator.keypad.set_pressed(keypad::Key::Key1);
          }
          glfw::WindowEvent::Key(Key::Num1, _, Action::Release, _) => {
            emulator.keypad.set_unpressed(keypad::Key::Key1);
          }
          glfw::WindowEvent::Key(Key::Num2, _, Action::Press, _) => {
            emulator.keypad.set_pressed(keypad::Key::Key2);
          }
          glfw::WindowEvent::Key(Key::Num2, _, Action::Release, _) => {
            emulator.keypad.set_unpressed(keypad::Key::Key2);
          },
          glfw::WindowEvent::Key(Key::Num3, _, Action::Press, _) => {
            emulator.keypad.set_pressed(keypad::Key::Key3);
          }
          glfw::WindowEvent::Key(Key::Num3, _, Action::Release, _) => {
            emulator.keypad.set_unpressed(keypad::Key::Key3);
          }
          glfw::WindowEvent::Key(Key::Q, _, Action::Press, _) => {
            println!("Aperto");
            emulator.keypad.set_pressed(keypad::Key::Key4);
          }
          glfw::WindowEvent::Key(Key::Q, _, Action::Release, _) => {
            println!("Solto");
            emulator.keypad.set_unpressed(keypad::Key::Key4);
          }
          glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
            emulator.keypad.set_pressed(keypad::Key::Key5);
          }
          glfw::WindowEvent::Key(Key::W, _, Action::Release, _) => {
            emulator.keypad.set_unpressed(keypad::Key::Key5);
          },
          glfw::WindowEvent::Key(Key::E, _, Action::Press, _) => {
            emulator.keypad.set_pressed(keypad::Key::Key6);
          }
          glfw::WindowEvent::Key(Key::E, _, Action::Release, _) => {
            emulator.keypad.set_unpressed(keypad::Key::Key6);
          }
          glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
            emulator.keypad.set_pressed(keypad::Key::Key7);
          }
          glfw::WindowEvent::Key(Key::A, _, Action::Release, _) => {
            emulator.keypad.set_unpressed(keypad::Key::Key7);
          }
          glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
            emulator.keypad.set_pressed(keypad::Key::Key8);
          }
          glfw::WindowEvent::Key(Key::S, _, Action::Release, _) => {
            emulator.keypad.set_unpressed(keypad::Key::Key8);
          }
          glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
            emulator.keypad.set_pressed(keypad::Key::Key9);
          }
          glfw::WindowEvent::Key(Key::D, _, Action::Release, _) => {
            emulator.keypad.set_unpressed(keypad::Key::Key9);
          }
          _ => {}
      }
  }
}
