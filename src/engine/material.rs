pub type Vector3 = cgmath::Vector3<f32>;

#[derive(Clone, Copy, Debug)]
pub struct Material {
  pub ambient: Vector3,
  pub diffuse: Vector3,
  pub specular: Vector3,
  pub shininess: f32
}

impl Material {
  pub const fn new(ambient: Vector3, diffuse: Vector3, specular: Vector3, shininess: f32) -> Self {
    Self {
      ambient, diffuse, specular, shininess
    }
  }
}