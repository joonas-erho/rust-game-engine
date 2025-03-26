use image::{self, GenericImageView};
use std::path::Path;
use cgmath::{vec2, vec3};
use tobj;

use super::mesh::{self, Mesh, Vertex};
use super::mesh::Texture;
use super::Shader;

#[derive(Default)]
pub struct Model {
  pub meshes: Vec<Mesh>,
  pub loaded_textures: Vec<Texture>,
  directory: String
}

impl Model {
  pub fn new(path: &str) -> Self {
    let mut instance = Model::default();

    instance.load_model(path);

    instance
  }

  pub fn draw(&self, shader: &Shader) -> () {
    for mesh in &self.meshes {
      mesh.draw(shader);
    }
  }

  pub fn load_model(&mut self, path: &str) -> () {
    let path = Path::new(path);

    // If path parent doesn't exist, make a new Path that's empty.
    self.directory = path.parent()
    .unwrap_or_else(
      || Path::new("")).to_str().unwrap().into();
    let loaded_obj = tobj::load_obj(path);

    let (models, materials) = loaded_obj.unwrap();
    if models.len() < 1 {
      dbg!("No models found in loaded object!");
      return;
    }

    for model in models {
      let mesh = &model.mesh;
      let vertex_count = mesh.positions.len() / 3;

      let mut vertices: Vec<Vertex> = Vec::with_capacity(vertex_count);
      let indices: Vec<u32> = mesh.indices.clone();

      let (positions, normals, tex_coords) =
        (&mesh.positions, &mesh.normals, &mesh.texcoords);
      for i in 0..vertex_count {
        vertices.push(Vertex {
          position: vec3(positions[i*3], positions[i*3+1], positions[i*3+2]),
          normal: vec3(normals[i*3], normals[i*3+1], normals[i*3+2]),
          tex_coords: vec2(tex_coords[i*2], tex_coords[i*2+1])
        })
      }

      let mut textures: Vec<Texture> = Vec::new();
      if let Some(material_id) = mesh.material_id {
        let material = &materials[material_id];

        if !material.diffuse_texture.is_empty() {
          let texture = self.load_material_texture(&material.diffuse_texture, "texture_diffuse");
          textures.push(texture);
        }
        
        if !material.specular_texture.is_empty() {
          let texture = self.load_material_texture(&material.specular_texture, "texture_specular");
          textures.push(texture);
        }
        if !material.normal_texture.is_empty() {
          let texture = self.load_material_texture(&material.normal_texture, "texture_normal");
          textures.push(texture);
        }
        
        // Todo: height maps?
      }

      self.meshes.push(Mesh::new(vertices, indices, textures));
    }
  }

  fn load_material_texture(&mut self, path: &str, texture_type: &str) -> Texture {
    let texture = self.loaded_textures.iter().find(|tex| tex.path == path);
    if let Some(texture) = texture {
      return texture.clone();
    } else {
      let texture = Texture {
        id: unsafe { Self::texture_from_file(path, &self.directory) },
        tex_type: texture_type.into(),
        path: path.into()
      };
      self.loaded_textures.push(texture.clone());
      texture
    }
  }

  unsafe fn texture_from_file(path: &str, dir: &str) ->  u32 {
    let file_name = format!("{}/{}", dir, path);

    let mut tex_id = 0;
    gl::GenTextures(1, &mut tex_id);

    let img = image::open(&Path::new(&file_name))
      .expect("Issue with loading texture!");

    // let img = img.flipv();
    let format = match img {
        image::DynamicImage::ImageLuma8(_) => gl::RED,
        image::DynamicImage::ImageLumaA8(_) => gl::RG,
        image::DynamicImage::ImageRgb8(_) => gl::RGB,
        image::DynamicImage::ImageRgba8(_) => gl::RGBA,
        image::DynamicImage::ImageLuma16(_) => todo!(),
        image::DynamicImage::ImageLumaA16(_) => todo!(),
        image::DynamicImage::ImageRgb16(_) => todo!(),
        image::DynamicImage::ImageRgba16(_) => todo!(),
        image::DynamicImage::ImageRgb32F(_) => todo!(),
        image::DynamicImage::ImageRgba32F(_) => todo!(),
        _ => todo!(),
    };

    gl::BindTexture(gl::TEXTURE_2D, tex_id);
    gl::TexImage2D(
      gl::TEXTURE_2D,
      0,
      format as i32,
      img.dimensions().0 as i32,
      img.dimensions().1 as i32,
      0,
      format,
      gl::UNSIGNED_BYTE,
      img.as_bytes().as_ptr() as *const _
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER,gl::LINEAR_MIPMAP_LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    tex_id
  }
}