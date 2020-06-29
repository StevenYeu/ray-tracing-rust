use std::io::{self, Write};
mod vector;
use vector::Vec3;
mod color;
use color::write_color;
fn main() -> io::Result<()> {
  let stdout = io::stdout();
  let mut handle = stdout.lock();
  let mut vector = Vec3::new(1.0, 3.0, 4.0);
  let add_vector = Vec3::new(2.0, 4.0, 5.0);
  vector += add_vector;
  let vector = 5.0 * vector;
  if let Err(e) = handle.write(format!("{},{},{}", vector.x(), vector.y(), vector.z()).as_bytes()) {
    return Err(e);
  }
  // let stderr = io::stderr();
  // let mut handle_err = stderr.lock();
  // let width = 256;
  // let height = 256;
  // if let Err(e) = handle.write(format!("P3\n{} {}\n255\n", width, height).as_bytes()) {
  //   return Err(e);
  // }
  // for j in (-1..(width - 1)).rev() {
  //   if let Err(e) = handle_err.write_all(format!("\rScanlines reamning: {} ", j).as_bytes()) {
  //     return Err(e);
  //   }
  //   for i in (0..height).rev() {
  //     let r = (i as f64) / ((width - 1) as f64);
  //     let g = (j as f64) / ((width - 1) as f64);
  //     let b = 0.25;

  //     let ir = (255.999 * r) as i64;
  //     let ib = (255.999 * b) as i64;
  //     let ig = (255.999 * g) as i64;

  //     if let Err(e) = handle.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()) {
  //       return Err(e);
  //     }
  //   }
  // }
  // if let Err(e) = handle_err.write_all(b"\nDone\n") {
  //   return Err(e);
  // }
  Ok(())
}
