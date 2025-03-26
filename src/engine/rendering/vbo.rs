use std::mem;

pub struct VBO {
  id: u32,
}

impl VBO {
  pub fn new() -> Self {
    let mut id = 0;
    unsafe {
      gl::GenBuffers(1, &mut id);
    }
    VBO { id }
  }

  pub unsafe fn bind(&self) {
    gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
  }

  pub unsafe fn unbind() {
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  }

  pub unsafe fn buffer_data<T>(&self, data: &[T]) {
    self.bind();
    let len = data.len() * mem::size_of::<T>();
    gl::BufferData(
      gl::ARRAY_BUFFER,
      len as isize,
      data.as_ptr() as *const _,
      gl::STATIC_DRAW,
    );
  }

  pub unsafe fn delete(&self) {
    gl::DeleteBuffers(1, &self.id);
  }
}
