#[allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use image::{self, GenericImageView};
use image::GenericImage;

use std::sync::mpsc::Receiver;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;
use std::os::raw::c_char;

use cgmath::{Matrix4, Vector3, vec3, Deg, Rad, perspective, ortho};
use cgmath::prelude::*;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 400;

const vertex_shader_source: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec2 aTexPos;

    out vec2 texCoords;

    uniform mat4 projection;
    uniform mat4 view;

    void main() {
      texCoords = aTexPos; 
      gl_Position = projection * view * vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const fragment_shader_source: &str = r#"
    #version 330 core
    in vec2 texCoords;
    out vec4 FragColor;

    uniform sampler2D myTexture;

    void main() {
      float color = texture(myTexture, texCoords).r;
      FragColor = vec4(color, color, color, 1.0f);
    }
"#;

pub fn run() -> (glfw::Window, Receiver<(f64, glfw::WindowEvent)>, glfw::Glfw, u32) {
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

    let shader_program = create_shader_program();

    unsafe {
      gl::UseProgram(shader_program);
      let perspective = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
      //let perspective = ortho(0.0, SCR_WIDTH as f32, 0.0, SCR_HEIGHT as f32, 0.1, 100.0);
      let view: Matrix4<f32> = Matrix4::from_translation(vec3(0., 0., -3.));
    
      let projection_loc = gl::GetUniformLocation(shader_program, CString::new("projection").unwrap().as_ptr());
      let view_loc = gl::GetUniformLocation(shader_program, CString::new("view").unwrap().as_ptr());

      gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, perspective.as_ptr());
      gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
    };

    (window, events, glfw, shader_program)
}

// NOTE: not the same version as in common.rs!


fn create_shader_program() -> GLuint {
  unsafe {
    // build and compile our shader program
    // ------------------------------------
    // vertex shader
    let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
    gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
    gl::CompileShader(vertex_shader);

    // check for shader compile errors
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
    gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
    }

    // fragment shader
    let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
    gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
    gl::CompileShader(fragment_shader);
    // check for shader compile errors
    gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
    }

    // link shaders
    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);
    // check for linking errors
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
    }
    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);

    shader_program
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

pub fn draw_rect(vertices: &mut Vec<f32>) {
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

pub fn set_texture(texture: &mut Vec<u8>, program_id: u32) {
  let mut tex= 0;
  unsafe { 
    gl::GenTextures(1, &mut tex); 
    gl::BindTexture(gl::TEXTURE_2D, tex);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    // set texture filtering parameters
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

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

    let tex_loc = gl::GetUniformLocation(program_id, CString::new("myTexture").unwrap().as_ptr());
    gl::Uniform1i(tex_loc, 0);
  };
}

pub fn draw_with_texure(vertices: &mut Vec<f32>) {
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