extern crate gl;
#[allow(dead_code)]
#[allow(unused_variables)]
extern crate glfw;

use cgmath::*;
use glfw::{Action, Context, GlfwReceiver, Key, OpenGlProfileHint, WindowHint, WindowMode};

mod engine;
use engine::{object::Object, screen::Screen, skybox::Skybox, *};

mod data;
use data::*;

use types::*;

static mut LOCK_MOUSE: bool = false;

fn main() {
  let window_width: u32 = 800;
  let window_height: u32 = 600;

  let mut delta_time: f64;
  let mut last_frame: f64 = 0.0;

  // let mut light_pos: Vector3<f32> = vec3(1.2, 1.0, 2.0);

  let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

  // These seem to have no impact on the functionality of the program,
  // but might as well have them anyway.
  glfw.window_hint(WindowHint::ContextVersion(3, 3));
  glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

  #[cfg(target_os = "macos")]
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

  let (mut window, events) = glfw
    .create_window(
      window_width,
      window_height,
      "Real-time Rendering, Chapter 4",
      WindowMode::Windowed,
    )
    .expect("Failed to create GLFW window.");

  let mut camera = Camera::new(Point3::new(0.0, 0.0, 3.0), window_width, window_height);

  window.set_key_polling(true);
  window.set_cursor_pos_polling(true);
  window.set_scroll_polling(true);
  window.make_current();
  window.set_scroll_callback(handle_scroll);
  window.set_framebuffer_size_callback(handle_resize);

  // This loads OpenGL function pointers (the ones used inside unsafe blocks)
  gl::load_with(|s| window.get_proc_address(s) as *const _);

  let mut cube_shader = Shader { id: 0 };
  cube_shader.initialize("shaders/default.vert", "shaders/default.frag");

  let mut skybox_shader = Shader { id: 1 };
  skybox_shader.initialize("shaders/skybox.vert", "shaders/skybox.frag");

  let cube = Object::new(
    Vertices::get_vertices(Vertices::Cube),
    "assets/images/marble.jpg",
    vec3(0.0, 0.0, 0.0),
  );

  let cube2 = Object::new(
    Vertices::get_vertices(Vertices::Cube),
    "assets/images/marble.jpg",
    vec3(5.0, 0.0, 0.0),
  );

  let cube3 = Object::new(
    Vertices::get_vertices(Vertices::Cube),
    "assets/images/marble.jpg",
    vec3(1.0, 2.0, 4.0),
  );

  let skybox = Skybox::new(
    Vertices::get_vertices(Vertices::Skybox),
    &[
      "assets/images/skybox/right.jpg",
      "assets/images/skybox/left.jpg",
      "assets/images/skybox/top.jpg",
      "assets/images/skybox/bottom.jpg",
      "assets/images/skybox/front.jpg",
      "assets/images/skybox/back.jpg",
    ],
  );

  cube_shader.activate();
  cube_shader.set_int("skybox", 0);

  skybox_shader.activate();
  skybox_shader.set_int("skybox", 0);

  unsafe {
    gl::Enable(gl::DEPTH_TEST);
    gl::DepthFunc(gl::ALWAYS);
    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
  }

  while !window.should_close() {
    let current_frame: f64 = glfw.get_time();
    delta_time = current_frame - last_frame;
    last_frame = current_frame;

    unsafe {
      gl::ClearColor(0.9, 0.9, 0.9, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

      let mut projection = Mat4::identity();
      projection = projection * perspective(Deg(camera.get_fov()), 800.0 / 600.0, 0.1, 100.0);

      cube_shader.activate();

      cube_shader.set_vec3("cameraPos", &camera.get_pos().to_vec());
      cube_shader.set_mat4("view", camera.get_view_matrix());
      cube_shader.set_mat4("projection", projection);

      cube.draw(&mut cube_shader);
      cube2.draw(&mut cube_shader);
      cube3.draw(&mut cube_shader);
      // plane.draw(&mut cube_shader);
      // window1.draw(&mut cube_shader);
      // window2.draw(&mut cube_shader);

      skybox_shader.activate();
      skybox.draw(&mut skybox_shader, &mut camera, projection);
    }

    handle_window_event(&mut window, delta_time, &mut camera, &events);

    window.swap_buffers();
    glfw.poll_events();

    let error = unsafe { gl::GetError() };
    if error != gl::NO_ERROR {
      println!("OpenGL Error: {}", error);
    }
  }

  unsafe {
    cube.delete();
    // plane.delete();
    gl::DeleteProgram(cube_shader.id);
  }

  drop(window);
  drop(glfw);

  fn handle_window_event(
    window: &mut glfw::Window,
    delta_time: f64,
    camera: &mut Camera,
    events: &GlfwReceiver<(f64, glfw::WindowEvent)>,
  ) {
    let camera_speed: f32 = 2.5 * delta_time as f32;

    if window.get_key(Key::Escape) == Action::Press {
      window.set_should_close(true)
    }

    if window.get_key(Key::Q) == Action::Press {
      unsafe {
        let mut poly_mode: i32 = 0;
        gl::GetIntegerv(gl::POLYGON_MODE, &mut poly_mode);
        if gl::LINE == poly_mode.try_into().unwrap() {
          gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        } else {
          gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
      }
    }

    if window.get_key(Key::L) == Action::Press {
      unsafe {
        LOCK_MOUSE = !LOCK_MOUSE;
      }
    }

    if window.get_key(Key::W) == Action::Press {
      camera.update_camera_pos_fb(camera_speed);
    }

    if window.get_key(Key::S) == Action::Press {
      camera.update_camera_pos_fb(-camera_speed);
    }

    if window.get_key(Key::A) == Action::Press {
      camera.update_camera_pos_lr(-camera_speed);
    }

    if window.get_key(Key::D) == Action::Press {
      camera.update_camera_pos_lr(camera_speed);
    }

    for (_, event) in glfw::flush_messages(events) {
      match event {
        glfw::WindowEvent::CursorPos(x, y) => unsafe {
          if !LOCK_MOUSE {
            camera.handle_mouse(x * 2.0, y * 2.0);
          }
        },
        glfw::WindowEvent::Scroll(_, scrl) => {
          camera.handle_scroll(scrl);
        }
        _ => {}
      }
    }
  }

  fn handle_scroll(_window: &mut glfw::Window, x_offset: f64, y_offset: f64) {}

  fn handle_resize(_window: &mut glfw::Window, width: i32, height: i32) {
    unsafe {
      gl::Viewport(0, 0, width, height);
    }
  }
}
