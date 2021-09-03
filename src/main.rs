use renderer;
use renderer::Renderer;

use cpu;
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

  let mut emulator = cpu::new(&String::from("Pong 2 (Pong hack) [David Winter, 1997].ch8"));
  let mut last_time = glfw.get_time();
  let mut framecount = 0;
  while !window.should_close() {
    let time = glfw.get_time();
    framecount += 1;

    if time - last_time >= 1.0 {
      println!("{}", framecount);
      framecount = 0;
      last_time = time;
    }
    
    glfw.poll_events();
    process_events(&mut window, &events);
    emulator.run_cicle();
    renderer.set_texture(&mut emulator.display.get_display_as_u8_texture());
    renderer.draw_with_texure(&mut vertices);
    renderer::swap_buffer(&mut window);
  };
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
  for (_, event) in glfw::flush_messages(events) {
      match event {
          glfw::WindowEvent::FramebufferSize(width, height) => {
            renderer::update_window_buffer(width, height);
          }
          glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
          glfw::WindowEvent::Key(Key::Right, _, Action::Press, _) => {
            println!("Direita boa");
          },
          _ => {}
      }
  }
}
