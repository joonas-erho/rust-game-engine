use std::mem;

use crate::data;
use crate::engine::Shader;
use cgmath::SquareMatrix;
use data::VertexArray;

use super::{
  material::Vector3,
  rendering::{VAO, VBO},
  Texture,
};

use super::types::Mat4;

#[derive(Clone)]
pub struct Object {
  pub vertex_array: VertexArray,
  pub vao: VAO,
  pub vbo: VBO,
  pub texture: Texture,
  pub position: Vector3,
}

impl Object {
  pub fn new(vertex_array: VertexArray, texture_path: &str, position: Vector3) -> Self {
    let instance = Object {
      vertex_array,
      vao: VAO::new(),
      vbo: VBO::new(),
      texture: Texture::new(texture_path),
      position,
    };

    unsafe {
      instance.vao.bind();
      instance.vbo.buffer_data(&instance.vertex_array.vertices);

      // Probably move this portion to the vbo struct
      let s = instance.vertex_array.stride as i32;
      let m = mem::size_of::<f32>();
      gl::EnableVertexAttribArray(0);
      gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, s, 0 as *const _);

      if instance.vertex_array.has_normals {
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, s, (3 * m) as *const _);
      }

      if instance.vertex_array.has_tex_coords {
        gl::EnableVertexAttribArray(2);
        let mut offset = 3 * m;
        if instance.vertex_array.has_normals {
          offset = 6 * m;
        }
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, s, offset as *const _);
      }

      VAO::unbind();
    }

    instance
  }

  pub fn draw(&self, shader: &mut Shader) -> () {
    unsafe {
      self.vao.bind();
      gl::BindTexture(gl::TEXTURE_2D, self.texture.id);
      let mut model = Mat4::identity();
      model = model * Mat4::from_translation(self.position);
      shader.set_mat4("model", model);
      gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_array.triangle_count);
    }
  }

  pub fn draw_outlines(&self, shader: &mut Shader, size: f32) -> () {
    unsafe {
      self.vao.bind();
      let mut model = Mat4::identity();
      model = model * Mat4::from_translation(self.position);
      model = model * Mat4::from_scale(size);
      shader.set_mat4("model", model);
      gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_array.triangle_count);
    }
  }

  pub fn delete(&self) -> () {
    unsafe {
      self.vao.delete();
      self.vbo.delete();
      gl::DeleteTextures(1, &self.texture.id);
    }
  }
}
