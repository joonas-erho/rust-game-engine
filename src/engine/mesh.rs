use std::mem::{self, offset_of};
use std::{ffi::CString};

use cgmath::*;

use super::Shader;

#[derive(Debug)]
#[repr(C)]
pub struct Vertex {
  pub position: Vector3<f32>,
  pub normal: Vector3<f32>,
  pub tex_coords: Vector2<f32>
}

#[derive(Clone, Debug)]
pub struct Texture {
  pub id: u32,
  pub tex_type: String,
  pub path: String
}

#[derive(Debug)]
pub struct Mesh {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u32>,
  pub textures: Vec<Texture>,
  vao: u32,
  vbo: u32,
  ebo: u32
}

impl Mesh {
  pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
    let mut instance = Self {
      vertices,
      indices,
      textures,
      vao: 0,
      vbo: 0,
      ebo: 0
    };

    instance.setup_mesh();

    instance
  }

  fn setup_mesh(&mut self) -> () {
    unsafe {
      gl::GenVertexArrays(1, &mut self.vao);
      gl::GenBuffers(1, &mut self.vbo);
      gl::GenBuffers(1, &mut self.ebo);

      gl::BindVertexArray(self.vao);

      gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (mem::size_of::<Vertex>() * self.vertices.len()) as isize,
        self.vertices.as_ptr().cast(),
        gl::STATIC_DRAW
      );

      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
      gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (mem::size_of::<u32>() * self.indices.len()) as isize,
        self.indices.as_ptr().cast(),
        gl::STATIC_DRAW
      );

      let mem_size = mem::size_of::<Vertex>() as i32;
      let stride = mem_size;
      gl::EnableVertexAttribArray(0);
      gl::VertexAttribPointer(
        0, 3, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, position) as *const _);
  
      gl::EnableVertexAttribArray(1);
      gl::VertexAttribPointer(
        1, 3, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, normal) as *const _);
  
      gl::EnableVertexAttribArray(2);
      gl::VertexAttribPointer(
        2, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, tex_coords) as *const _);

      gl::BindVertexArray(0);
    }
  }

  pub fn draw(&self, shader: &Shader) -> () {
    let mut diffuse_nr = 0;
    let mut specular_nr = 0;
    let mut normal_nr = 0;
    let mut height_nr = 0;
    unsafe {
      for i in 0..self.textures.len() {
        gl::ActiveTexture(gl::TEXTURE0 + (i as u32));
        let name = &self.textures[i].tex_type;
        let number = match name.as_str() {
          "texture_diffuse" => {
            diffuse_nr += 1;
            diffuse_nr
          }
          "texture_specular" => {
            specular_nr += 1;
            specular_nr
          }
          "texture_normal" => {
            normal_nr += 1;
            normal_nr
          }
          "texture_height" => {
            height_nr += 1;
            height_nr
          }
          _ => panic!("Texture type could not be determined!")
        };

        gl::Uniform1i(
          gl::GetUniformLocation(
            shader.id,
            CString::new(format!("{}{}", name, number)).unwrap().into_raw()), i as i32);
        gl::BindTexture(gl::TEXTURE_2D, self.textures[i].id);
      }
      gl::BindVertexArray(self.vao);
      gl::DrawElements(gl::TRIANGLES, self.indices.len().try_into().unwrap(), gl::UNSIGNED_INT, std::ptr::null());
      gl::BindVertexArray(0);
      gl::ActiveTexture(gl::TEXTURE0);
    }
  }
}