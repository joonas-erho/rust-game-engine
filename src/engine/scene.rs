use cgmath::vec3;

use super::{object::Object, types::Vec3, Texture};

/// A scene compasses all game objects in it, as well as settings
/// such as skybox/bg color, general lighting and physics (todo).
/// 
/// It is a scene's responsibility to handle its children and 
/// render all of its contents.
/// 
/// TODO: It is possible to load multiple scenes at once.
pub struct Scene {
  pub objects: Vec<Object>,
  pub has_skybox: bool,
  pub skybox_tex: Option<Texture>,
  pub bg_color: Vec3
}

impl Scene {
  pub fn new() -> Self {
    Scene {
      objects: [].to_vec(),
      has_skybox: false,
      skybox_tex: None,
      bg_color: vec3(0.5, 0.5, 0.5)
    }
  }
}