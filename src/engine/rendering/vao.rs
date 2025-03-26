pub struct VAO {
  id: u32,
}

impl VAO {
  pub fn new() -> Self {
    let mut id = 0;
    unsafe {
      gl::GenVertexArrays(1, &mut id);
    }
    VAO { id }
  }

  pub unsafe fn bind(&self) {
    gl::BindVertexArray(self.id);
  }

  pub unsafe fn unbind() {
    gl::BindVertexArray(0);
  }

  pub unsafe fn delete(&self) {
    gl::DeleteVertexArrays(1, &self.id);
  }
}
