use renderer;
extern crate glfw;

use self::glfw::{Action, Key};

use std::sync::mpsc::Receiver;

fn translate_geo(vertices: &mut Vec<f32>, translation: f32) -> Vec<f32> {
  let translated: Vec<f32> = vertices.into_iter().map(|v| *v + translation).collect();
  translated
}

static mut translate: f32 = 0.0;

fn main() {
  let (mut window, events, mut glfw) = renderer::run();
  let mut vertices: Vec<f32> = vec![
    -0.5, 0.5, 0.0,
    0.5, 0.5, 0.0,
    -0.5, -0.5, 0.0,
    0.5, 0.5, 0.0,
    0.5, -0.5, 0.0,
    -0.5, -0.5, 0.0,
  ];

  while !window.should_close() {
    glfw.poll_events();
    process_events(&mut window, &events);
    let mut translated = unsafe { translate_geo(&mut vertices, translate)};
    let mut translated2 = unsafe { translate_geo(&mut vertices, 0.5)};
    renderer::draw_rect(&mut translated);
    renderer::draw_rect(&mut translated2);
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
            unsafe {translate = translate + 0.01};
            unsafe {println!("{}", translate);};
          },
          _ => {}
      }
  }
}
