use crate::engine;
use engine::material::*;

pub enum Materials {
  Emerald,
  Pearl,
  Copper,
  RedPlastic,
  YellowRubber
}

impl Materials {
  pub fn get_material(self) -> Material {
    match self {
      Materials::Emerald => Material::new(
        Vector3::new(0.0215, 0.1745, 0.0215),
        Vector3::new(0.07568, 0.61424, 0.07568),
        Vector3::new(0.633, 0.727811, 0.633),
        0.6,
      ),
      Materials::Pearl => Material::new(
        Vector3::new(0.25, 0.20725, 0.20725),
        Vector3::new(1.0, 0.829, 0.829),
        Vector3::new(0.296648, 0.296648, 0.296648),
        0.088,
      ),
      Materials::Copper => Material::new(
        Vector3::new(0.19125, 0.0735, 0.0225),
        Vector3::new(0.7038, 0.27048, 0.0828),
        Vector3::new(0.256777, 0.137622, 0.086014),
        0.1,
      ),
      Materials::RedPlastic => Material::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.5, 0.0, 0.0),
        Vector3::new(0.7, 0.6, 0.6),
        0.25,
      ),
      Materials::YellowRubber => Material::new(
        Vector3::new(0.05, 0.05, 0.0),
        Vector3::new(0.5, 0.5, 0.4),
        Vector3::new(0.7, 0.7, 0.04),
        0.078125,
      ),
    }
  }
}
