use std::{ffi::CString, fs::read_to_string, ptr::{null, null_mut}};

use cgmath::{Array, Matrix};
use gl::types::GLenum;

use super::types::*;

#[derive(Debug)]
pub struct Shader {
  pub id: u32
}

impl Shader {
  pub fn initialize(&mut self, path_to_vertex_shader: &str, path_to_fragment_shader: &str) -> () {
    // dbg!(path_to_vertex_shader);
    let binding = read_to_string(path_to_vertex_shader)
    .expect("Issue with reading vertex shader!");
    let vert_shader = binding.as_str();
    let binding = read_to_string(path_to_fragment_shader)
    .expect("Issue with reading vertex shader!");
    let frag_shader = binding.as_str();
    // dbg!(vert_shader);
    // dbg!(frag_shader);
    let vert_id = self.create_shader(vert_shader, gl::VERTEX_SHADER);
    let frag_id = self.create_shader(frag_shader, gl::FRAGMENT_SHADER);
    self.id = self.create_program(vert_id, frag_id);
    self.delete_shader(vert_id);
    self.delete_shader(frag_id);
  }

  fn create_shader(&mut self, shader: &str, shader_type: GLenum) -> u32 {
    let mut success: i32 = 0;
    let mut info_log: [i8; 512] = [0; 512];
    let id;

    unsafe {
      id = gl::CreateShader(shader_type);
      let c_str = std::ffi::CString::new(shader).expect("Failed to create CString");
      gl::ShaderSource(id, 1, &(c_str.as_ptr().cast()), null());
      gl::CompileShader(id);
      gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
      if success == 0 {
        gl::GetShaderInfoLog(id, 512, null_mut(), info_log.as_mut_ptr());
        println!("Error in initializing shader!");
      let log_as_u8: Vec<u8> = info_log.iter().map(|&x| x as u8).collect();
      println!("{}", std::str::from_utf8(&log_as_u8).unwrap());
      } else {
        println!("Succesfully generated shader. Shader ID is {}", id)
      }
    }

    return id
  }

  fn create_program(&mut self, vert_id: u32, frag_id: u32) -> u32 {
    let mut success: i32 = 0;
    let mut info_log: [i8; 512] = [0; 512];
    let program_id;

    unsafe {
      program_id = gl::CreateProgram();
      gl::AttachShader(program_id, vert_id);
      gl::AttachShader(program_id, frag_id);
      gl::LinkProgram(program_id);
      gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
      if success == 0 {
        gl::GetProgramInfoLog(program_id, 512, null_mut(),
        info_log.as_mut_ptr());
        println!("Error in initializing shader program!");
        let log_as_u8: Vec<u8> = info_log.iter().map(|&x| x as u8).collect();
        println!("{}", std::str::from_utf8(&log_as_u8).unwrap());
      } else {
        println!("Succesfully generated shader program and linked shaders to it. Program ID is {}", program_id)
      }
    }

    return program_id
  }

  pub fn activate(&mut self) -> () {
    unsafe {
      gl::UseProgram(self.id);
    }
  }

  pub fn set_bool(&mut self, name: &str, value: bool) -> () {
    unsafe {
      gl::Uniform1i(
        gl::GetUniformLocation(
          self.id, CString::new(name).unwrap().into_raw()),
          value as i32);
    }
  }

  pub fn set_int(&mut self, name: &str, value: i32) -> () {
    unsafe {
      gl::Uniform1i(
        gl::GetUniformLocation(
          self.id, CString::new(name).unwrap().into_raw()),
          value);
    }
  }

  pub fn set_float(&mut self, name: &str, value: f32) -> () {
    unsafe {
      gl::Uniform1f(
        gl::GetUniformLocation(
          self.id, CString::new(name).unwrap().into_raw()),
          value);
    }
  }

  pub fn set_vec3(&mut self, name: &str, value: &Vec3) -> () {
    unsafe {
      gl::Uniform3fv(
        gl::GetUniformLocation(self.id, CString::new(name).unwrap().into_raw()),
        1, value.as_ptr()
      );
    }
  }

  pub fn set_mat4(&mut self, name: &str, value: Mat4) -> () {
    unsafe {
      gl::UniformMatrix4fv(
        gl::GetUniformLocation(
          self.id, CString::new(name).unwrap().into_raw()),
          1, gl::FALSE, value.as_ptr());
    }
  }

  fn delete_shader(&mut self, shader_id: u32) -> () {
    unsafe {
      gl::DeleteShader(shader_id);
    }
  }
}