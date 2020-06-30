use super::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
  println!(
    "{} {} {}",
    (255.99 * pixel_color.x()) as i64,
    (255.99 * pixel_color.y()) as i64,
    (255.99 * pixel_color.z()) as i64
  )
}
