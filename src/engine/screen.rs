use std::mem;

use crate::engine::Shader;
use crate::data;
use cgmath::{vec3, SquareMatrix};
use data::VertexArray;

use super::{material::Vector3, Texture};

type Mat4 = cgmath::Matrix4<f32>;

pub struct Screen {
  pub vertex_array: VertexArray,
  pub vao: u32,
  pub vbo: u32,
}

impl Screen {
  pub fn new(vertex_array: VertexArray) -> Self {
    let mut instance = Screen {
      vertex_array,
      vao: 0,
      vbo: 0,
    };

    unsafe {
      gl::GenVertexArrays(1, &mut instance.vao);
      gl::GenBuffers(1, &mut instance.vbo);
      gl::BindVertexArray(instance.vao);
      gl::BindBuffer(gl::ARRAY_BUFFER, instance.vbo);
      let len = &instance.vertex_array.vertices.len() * mem::size_of::<f32>();
      gl::BufferData(
        gl::ARRAY_BUFFER,
        len as isize,
        instance.vertex_array.vertices.as_ptr().cast(),
        gl::STATIC_DRAW
      );

      let s = instance.vertex_array.stride as i32;
      let m = mem::size_of::<f32>();
      gl::EnableVertexAttribArray(0);
      gl::VertexAttribPointer(
        0, 2, gl::FLOAT, gl::FALSE, s, 0 as *const _
      );

      if instance.vertex_array.has_normals {
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
          1, 3, gl::FLOAT, gl::FALSE, s, (2 * m) as *const _
        );
      }

      if instance.vertex_array.has_tex_coords {
        gl::EnableVertexAttribArray(2);
        let mut offset = 2 * m;
        if instance.vertex_array.has_normals {
          offset = 4 * m;
        }
        gl::VertexAttribPointer(
          2, 2, gl::FLOAT, gl::FALSE, s, offset as *const _
        );
      }

      gl::BindVertexArray(0);
    }

    instance
  }

  pub fn draw(&self, shader: &mut Shader) -> () {
    unsafe {
      gl::BindVertexArray(self.vao);
      let model = Mat4::identity();
      shader.set_mat4("model", model);
      gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_array.triangle_count);
    }
  }

  pub fn draw_outlines(&self, shader: &mut Shader, size: f32) -> () {
    unsafe {
      gl::BindVertexArray(self.vao);
      let mut model = Mat4::identity();
      model = model * Mat4::from_scale(size);
      shader.set_mat4("model", model);
      gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_array.triangle_count);
    }
  }

  pub fn delete(&self) -> () {
    unsafe {
      gl::DeleteVertexArrays(1, &self.vao);
      gl::DeleteBuffers(1, &self.vbo);
    }
  }
}