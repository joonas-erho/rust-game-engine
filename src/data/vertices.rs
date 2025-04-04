#[derive(Clone)]
pub struct VertexArray {
  // Vertices of the object. This always includes position vertices, but
  // may also include normals and texture coordinates.
  pub vertices: Vec<f32>,

  // Does the vertices -array contain normals?
  pub has_normals: bool,

  // Does the vertices -array contain texture coordinates?
  pub has_tex_coords: bool,

  // The stride of the vertices array. It should be noted that this is
  // multiplied by the size of f32 in bytes (4), so only the amount of floats
  // should be given.
  pub stride: u8,

  // Triangle count of the rendering itself. For example, if this is 2, then
  // the renderer will expect to render two triangles.
  pub triangle_count: i32
}

impl VertexArray {
  pub fn new(vertices: &[f32], stride: u8, has_normals: bool, has_tex_coords: bool, triangle_count: i32) -> Self {
    Self {
      vertices: vertices.to_vec(),
      has_normals,
      has_tex_coords,
      stride: stride * 4,
      triangle_count
    }
  }
}

// Used in object creation. The values of these enums are defined below.
pub enum Vertices {
  Cube,
  CubeWithoutNormals,
  PlaneWithoutNormals,
  FlatImage, // Used to render the grass texture in the Blending chapter
  Screen, // Used to render the framebuffer texture
  ReverseCamera, // For the reverse camera exercise in the Framebuffers chapter
  Skybox
}

impl Vertices {
  pub fn get_vertices(self) -> VertexArray {
    match self {
      Vertices::Cube => VertexArray::new(
        &CUBE_VERTICES,
        8,
        true,
        true,
        36
      ),
      Vertices::CubeWithoutNormals => VertexArray::new(
        &CUBE_VERTICES_NO_NORMALS,
        5,
        false,
        true,
        36
      ),
      Vertices::PlaneWithoutNormals => VertexArray::new(
        &PLANE_VERTICES,
        5,
        false,
        true,
        6
      ),
      Vertices::FlatImage => VertexArray::new(
        &FLAT_IMAGE_VERTICES,
        5,
        false,
        true,
        6
      ),
      Vertices::Screen => VertexArray::new(
        &QUAD_VERTICES,
        4,
        false,
        true,
        6
      ),
      Vertices::ReverseCamera => VertexArray::new(
        &REVERSE_CAMERA_VERTICES,
        4,
        false,
        true,
        6
      ),
      Vertices::Skybox => VertexArray::new(
        &SKYBOX,
        3,
        false,
        false,
        36
      )
    }
  }
}

const CUBE_VERTICES: [f32; 288] = [
 -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,
  0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 0.0,
  0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
  0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
 -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 1.0,
 -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,

 -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0, 0.0,
  0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 0.0,
  0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 1.0,
  0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 1.0,
 -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0, 1.0,
 -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0, 0.0,

 -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,
 -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0, 1.0,
 -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
 -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
 -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0, 0.0,
 -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,

  0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,
  0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0, 1.0,
  0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
  0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
  0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0, 0.0,
  0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,

 -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,
  0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0, 1.0,
  0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
  0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
 -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0, 0.0,
 -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,

 -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0,
  0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0, 1.0,
  0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
  0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
 -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0, 0.0,
 -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0
];

const CUBE_VERTICES_NO_NORMALS: [f32; 180] = [
    -0.5, -0.5, -0.5,  0.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5, -0.5, -0.5,  1.0, 0.0,        
     0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 0.0,
    -0.5,  0.5, -0.5,  0.0, 1.0,

    -0.5, -0.5,  0.5,  0.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
    -0.5,  0.5,  0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,

    -0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5,  0.5,  1.0, 0.0,

     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,        
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5, -0.5,  0.5,  0.0, 0.0,    

    -0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  1.0, 1.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,

    -0.5,  0.5, -0.5,  0.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,    
     0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5, -0.5,  0.0, 1.0,
    -0.5,  0.5,  0.5,  0.0, 0.0
];

const PLANE_VERTICES: [f32; 30] = [
  -5.0, -0.5, -5.0,  0.0, 2.0,
  -5.0, -0.5,  5.0,  0.0, 0.0,
   5.0, -0.5,  5.0,  2.0, 0.0,

   5.0, -0.5, -5.0,  2.0, 2.0,
  -5.0, -0.5, -5.0,  0.0, 2.0,
   5.0, -0.5,  5.0,  2.0, 0.0
];

const FLAT_IMAGE_VERTICES: [f32; 30] = [
  0.0,  0.5,  0.0,  1.0,  1.0,
  0.0, -0.5,  0.0,  1.0,  0.0,
  1.0, -0.5,  0.0,  0.0,  0.0,
  0.0,  0.5,  0.0,  1.0,  1.0,
  1.0, -0.5,  0.0,  0.0,  0.0,
  1.0,  0.5,  0.0,  0.0,  1.0
];

const QUAD_VERTICES: [f32; 24] = [
  -1.0,  1.0,  0.0, 1.0,
  -1.0, -1.0,  0.0, 0.0,
   1.0, -1.0,  1.0, 0.0,
  -1.0,  1.0,  0.0, 1.0,
   1.0, -1.0,  1.0, 0.0,
   1.0,  1.0,  1.0, 1.0
];

const REVERSE_CAMERA_VERTICES: [f32; 24] = [
  -1.0, -0.7,  0.0, 1.0,
  -1.0, -1.0,  0.0, 0.0,
   -0.7, -1.0,  1.0, 0.0,
   -1.0, -0.7,  0.0, 1.0,
   -0.7, -1.0,  1.0, 0.0,
   -0.7, -0.7,  1.0, 1.0
];

const SKYBOX: [f32; 108] = [
  -1.0,  1.0, -1.0,
  -1.0, -1.0, -1.0,
  1.0, -1.0, -1.0,
  1.0, -1.0, -1.0,
  1.0,  1.0, -1.0,
  -1.0,  1.0, -1.0,

  -1.0, -1.0,  1.0,
  -1.0, -1.0, -1.0,
  -1.0,  1.0, -1.0,
  -1.0,  1.0, -1.0,
  -1.0,  1.0,  1.0,
  -1.0, -1.0,  1.0,

  1.0, -1.0, -1.0,
  1.0, -1.0,  1.0,
  1.0,  1.0,  1.0,
  1.0,  1.0,  1.0,
  1.0,  1.0, -1.0,
  1.0, -1.0, -1.0,

  -1.0, -1.0,  1.0,
  -1.0,  1.0,  1.0,
  1.0,  1.0,  1.0,
  1.0,  1.0,  1.0,
  1.0, -1.0,  1.0,
  -1.0, -1.0,  1.0,

  -1.0,  1.0, -1.0,
  1.0,  1.0, -1.0,
  1.0,  1.0,  1.0,
  1.0,  1.0,  1.0,
  -1.0,  1.0,  1.0,
  -1.0,  1.0, -1.0,

  -1.0, -1.0, -1.0,
  -1.0, -1.0,  1.0,
  1.0, -1.0, -1.0,
  1.0, -1.0, -1.0,
  -1.0, -1.0,  1.0,
  1.0, -1.0,  1.0
];