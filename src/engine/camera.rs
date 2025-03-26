use cgmath::{vec3, InnerSpace};

use super::types::*;

static mut FIRST_MOUSE_MOVEMENT: bool = true;

#[derive(Debug)]
pub struct Camera {
  camera_pos: Point3,
  camera_front: Vec3,
  camera_up: Vec3,
  camera_right: Vec3,
  world_up: Vec3,
  pub yaw: f32,
  pitch: f32,
  last_x: f32,
  last_y: f32,
  fov: f32
}

impl Camera {
  pub fn new(camera_pos: Point3, win_w: u32, win_h: u32) -> Self {
    Self { 
      camera_pos,
      camera_front: vec3(0.0, 0.0, -1.0),
      camera_up: vec3(0.0, 1.0, 0.0),
      camera_right: vec3(0.0, 0.0, 0.0),
      world_up: vec3(0.0, 1.0, 0.0),
      yaw: -90.0,
      pitch: 0.0,
      last_x: win_w as f32 / 2.0,
      last_y: win_h as f32 / 2.0,
      fov: 45.0
    }
  }

  pub fn get_pos(&mut self) -> Point3 {
    return self.camera_pos;
  }

  pub fn get_front(&mut self) -> Vec3 {
    return self.camera_front;
  }

  pub fn get_view_matrix(&mut self) -> Mat4 {
    return Mat4::look_at_rh(self.camera_pos, self.camera_pos + self.camera_front, self.camera_up);
  }

  // Updates camera position when moving front (f) or back (b)
  pub fn update_camera_pos_fb(&mut self, change: f32) -> () {
    self.camera_pos += self.camera_front * change;
  }

  // Updates camera position when moving left (l) or right (r)
  pub fn update_camera_pos_lr(&mut self, change: f32) -> () {
    self.camera_pos += self.camera_front.cross(self.camera_up) * change;
  }

  pub fn update_coordinates(&mut self, x: f32, y: f32) -> () {
    self.last_x = x;
    self.last_y = y;
  }

  pub fn handle_mouse(&mut self, x_pos: f64, y_pos: f64) {
    let x: f32 = x_pos as f32;
    let y: f32 = y_pos as f32;

    unsafe {
      if FIRST_MOUSE_MOVEMENT {
        self.last_x = x;
        self.last_y = y;
        FIRST_MOUSE_MOVEMENT = false;
      }
    }

    let mut x_offset = x - self.last_x;
    let mut y_offset = self.last_y - y;
    self.last_x = x;
    self.last_y = y;

    let sens = 0.1;
    x_offset *= sens;
    y_offset *= sens;

    self.yaw += x_offset;
    self.pitch += y_offset;

    if self.pitch > 89.0 {
      self.pitch = 89.0;
    }
    else if self.pitch < -89.0 {
      self.pitch = -89.0;
    }

    self.update_camera_pos();
  }

  pub fn handle_scroll(&mut self, scroll: f64) {
    self.fov -= scroll as f32;

    if self.fov < 1.0 {
      self.fov = 1.0;
    }
    else if self.fov > 65.0 {
      self.fov = 65.0;
    }
  }

  pub fn update_camera_pos(&mut self) {
    let mut front: Vec3 = vec3(0.0,0.0,0.0);
    front.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
    front.y = self.pitch.to_radians().sin();
    front.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

    self.camera_front = front.normalize();
    self.camera_right = self.camera_front.cross(self.world_up).normalize();
    self.camera_up = self.camera_right.cross(self.camera_front).normalize();
  }

  pub fn get_fov(&mut self) -> f32 {
    return self.fov;
  }
}