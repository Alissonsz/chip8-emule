use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use gl;
use gl::types::*;

use cgmath::{Matrix, Matrix4, Vector3};
use cgmath::prelude::*;

pub struct Shader {
  pub program_id: u32
}

impl Shader {
  pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
    unsafe {
      // vertex shader
      let mut vs_file = File::open(vertex_path)
                              .unwrap_or_else(|_| panic!("Failed to open {}", vertex_path));
      let mut v_code = String::new(); 
      vs_file.read_to_string(&mut v_code)
             .expect("Failed to read vertex shader");
      let vs_code = CString::new(v_code.as_bytes()).unwrap();

      // frag shader
      let mut fs_file = File::open(fragment_path)
                              .unwrap_or_else(|_| panic!("Failed to open {}", fragment_path));
      let mut f_code = String::new(); 
      fs_file.read_to_string(&mut f_code)
             .expect("Failed to read fragment shader");
      let fs_code = CString::new(f_code.as_bytes()).unwrap();
      
      let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
      gl::ShaderSource(vertex_shader, 1, &vs_code.as_ptr(), ptr::null());
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
      gl::ShaderSource(fragment_shader, 1, &fs_code.as_ptr(), ptr::null());
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
  
      return Shader {
        program_id: shader_program
      };
    }
  }

  pub fn set_mat4(&self, matrix: &Matrix4<f32>, name: &CStr) {
    unsafe {
      gl::UniformMatrix4fv(gl::GetUniformLocation(self.program_id, name.as_ptr()), 1, gl::FALSE, matrix.as_ptr());
    }
  }

  pub fn set_1i(&self, value: i32, name: &CStr) {
    unsafe {
      gl::Uniform1i(gl::GetUniformLocation(self.program_id, name.as_ptr()), value);
    }
  }
}