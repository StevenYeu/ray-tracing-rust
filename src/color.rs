// mod vector;
use std::io::{self, Write};

type Color = Vec3;

pub fn write_color(pixel_color: Color) -> io::Result<()> {
  let stdout = io::stdout();
  let mut handle = stdout.lock();
  if let Err(e) = handle.write(
    format!(
      "{} {} {}\n",
      (255.99 * pixel_color.x()) as i64,
      (255.99 * pixel_color.y()) as i64,
      (255.99 * pixel_color.z()) as i64
    )
    .as_bytes(),
  ) {
    return Err(e);
  }
  Ok(())
}
