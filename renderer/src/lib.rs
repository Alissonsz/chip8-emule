#[allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use image::{self, GenericImageView};
use image::GenericImage;

use std::sync::mpsc::Receiver;
use std::ffi::{CString, CStr};
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;
use std::os::raw::c_char;

use cgmath::{Matrix4, Vector3, vec3, Deg, Rad, perspective, ortho};
use cgmath::prelude::*;

mod shader;
use shader::Shader;

// settings
const SCR_WIDTH: u32 = 640;
const SCR_HEIGHT: u32 = 320;

pub struct Renderer {
  shader: Shader
}

impl Renderer {
  pub fn new() -> (Renderer, Receiver<(f64, glfw::WindowEvent)>, glfw::Window, glfw::Glfw) {
    // glfw: initialize and configure
    // ------------------------------

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    //glfw.window_hint(glfw::WindowHint::DoubleBuffer(false));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "CHIP8 EMULATOR", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let shader_program = Shader::new("renderer/src/shaders/vertex.vs", "renderer/src/shaders/fragment.fs");
    
    unsafe { gl::UseProgram(shader_program.program_id) };
    
    let perspective = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
    //let perspective = ortho(0.0, SCR_WIDTH as f32, 0.0, SCR_HEIGHT as f32, 0.1, 100.0);
    let view: Matrix4<f32> = Matrix4::from_translation(vec3(0., 0., -3.));
  
    shader_program.set_mat4(&perspective, &CString::new("projection").unwrap());
    shader_program.set_mat4(&view, &CString::new("view").unwrap());
    
    let renderer = Renderer {
      shader: shader_program,
    };

    (renderer, events, window, glfw)
  }

  pub fn draw_rect(&self, vertices: &mut Vec<f32>) {
    unsafe {  
      let (mut vbo, mut vao) = (0, 0);
      gl::GenVertexArrays(1, &mut vao);
      gl::GenBuffers(1, &mut vbo);
      // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
      gl::BindVertexArray(vao);

      gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
      gl::BufferData(gl::ARRAY_BUFFER,
                    (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr,
                    vertices.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW);

      gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
      gl::EnableVertexAttribArray(0);

      let vertices_count = vertices.len() as i32;

      // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
      gl::DrawArrays(gl::TRIANGLES, 0, vertices_count);
      // You can unbind the vao afterwards so other vao calls won't accidentally modify this vao, but this rarely happens. Modifying other
      // vaos requires a call to glBindVertexArray anyways so we generally don't unbind vaos (nor vbos) when it's not directly necessary.
      gl::BindVertexArray(0);
    };
  }

  pub fn set_texture(&self, texture: &mut Vec<u8>) {
    let mut tex= 0;
    unsafe { 
      gl::GenTextures(1, &mut tex); 
      gl::BindTexture(gl::TEXTURE_2D, tex);

      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::NEAREST as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::NEAREST as i32);
      // set texture filtering parameters
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

      gl::TexImage2D(
        gl::TEXTURE_2D, 
        0, 
        gl::RED as i32, 
        64 as i32, 
        32 as i32, 
        0, 
        gl::RED, 
        gl::UNSIGNED_BYTE, 
        &texture[0] as *const u8 as *const c_void
      );

      self.shader.set_1i(0, &CString::new("myTexture").unwrap());
    };
  }

  pub fn draw_with_texure(&self, vertices: &mut Vec<f32>) {
    unsafe {  
      let (mut vbo, mut vao) = (0, 0);
      gl::GenVertexArrays(1, &mut vao);
      gl::GenBuffers(1, &mut vbo);
      // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
      gl::BindVertexArray(vao);

      gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
      gl::BufferData(gl::ARRAY_BUFFER,
                    (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr,
                    vertices.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW);

      let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;

      gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
      gl::EnableVertexAttribArray(0);

      gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as * const c_void);
      gl::EnableVertexAttribArray(1);

      let vertices_count = 6 as i32;

      // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
      gl::DrawArrays(gl::TRIANGLES, 0, vertices_count);
      // You can unbind the vao afterwards so other vao calls won't accidentally modify this vao, but this rarely happens. Modifying other
      // vaos requires a call to glBindVertexArray anyways so we generally don't unbind vaos (nor vbos) when it's not directly necessary.
      gl::BindVertexArray(0);
    };
  }
}

pub fn swap_buffer(window: &mut glfw::Window) {
  window.swap_buffers();
  unsafe {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
  };
}

pub fn update_window_buffer(width: i32, height: i32) {
  unsafe { gl::Viewport(0, 0, width, height) };
}
