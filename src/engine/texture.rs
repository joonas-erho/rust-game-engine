use image::{self, GenericImageView, ImageError};

#[derive(Debug)]
pub struct Texture {
  pub id: u32
}

impl Texture {
  pub fn new(path_to_image_file: &str) -> Self {
    let mut instance = Texture { id: 0 };

    let _ = instance.generate(path_to_image_file);

    instance
  }

  pub fn generate(&mut self, path_to_image_file: &str) -> Result<(), ImageError> {
    let mut img = image::open(path_to_image_file).unwrap();
    
    // Add handling here in case img is not properly loaded.
    // println!("{:?}", img.dimensions());

    let extension = path_to_image_file.rsplit('.').next();
    
    if extension == Some("png") {
      img = img.flipv();
    }

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

    unsafe {
      gl::GenTextures(1, &mut self.id);
      gl::BindTexture(gl::TEXTURE_2D, self.id);
      
      gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        format as i32,
        img.dimensions().0 as i32,
        img.dimensions().1 as i32,
        0,
        format, gl::UNSIGNED_BYTE,
        img.as_bytes().as_ptr() as *const _
      );
      gl::GenerateMipmap(gl::TEXTURE_2D);

      if format == gl::RGBA {
        gl::TexParameteri(
          gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(
          gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
      }
      else {
        gl::TexParameteri(
          gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(
          gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
      }
      gl::TexParameteri(
        gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER,
        gl::LINEAR_MIPMAP_LINEAR as i32);
      gl::TexParameteri(
        gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }
    Ok(())
  }
}