use std::mem;
use image::GenericImageView;

use crate::data::VertexArray;

use super::{types::Mat4, Camera, Shader};

pub struct Skybox {
  pub vertex_array: VertexArray,
  pub vao: u32,
  pub vbo: u32,
  pub cubemap_texture: u32
}

impl Skybox {
  pub fn new(vertex_array: VertexArray, texture_paths: &[&str]) -> Self {
    let mut instance = Skybox {
      vertex_array,
      vao: 0,
      vbo: 0,
      cubemap_texture: 0
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
        0, 3, gl::FLOAT, gl::FALSE, s, 0 as *const _
      );

      gl::BindVertexArray(0);

      instance.cubemap_texture = instance.load_cubemap(texture_paths);
    }

    instance
  }

  pub fn draw(&self, shader: &mut Shader, camera: &mut Camera, projection: Mat4) -> () {
    unsafe {
      gl::DepthFunc(gl::LEQUAL);
      shader.activate();
      let mut view = camera.get_view_matrix();
      view.w[0] = 0.0;
      view.w[1] = 0.0;
      view.w[2] = 0.0;
      shader.set_mat4("view", view);
      shader.set_mat4("projection", projection);
      gl::BindVertexArray(self.vao);
      gl::ActiveTexture(gl::TEXTURE0);
      gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.cubemap_texture);
      gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_array.triangle_count);
      gl::DepthFunc(gl::LESS);
    }
  }


  unsafe fn load_cubemap(&self, texture_paths: &[&str]) -> u32 {
    let mut tex_id = 0;
    gl::GenTextures(1, &mut tex_id);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, tex_id);

    for (i, f) in texture_paths.iter().enumerate() {
      let img = image::open(f).expect("Failed to load texture image");

      gl::TexImage2D(
        gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
        0,
        gl::RGB as i32,
        img.dimensions().0 as i32,
        img.dimensions().1 as i32,
        0,
        gl::RGB, gl::UNSIGNED_BYTE,
        img.as_bytes().as_ptr() as *const _
      );
    }

    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);

    tex_id
  }
}