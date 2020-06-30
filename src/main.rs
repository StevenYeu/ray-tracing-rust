mod vector;
use vector::color::write_color;
use vector::color::Color;
fn main() {
  let width = 256;
  let height = 256;
  println!("P3\n{} {}\n255", width, height);

  for j in (-1..(width - 1)).rev() {
    eprintln!("\rScanlines reamning: {} ", j);
    for i in (0..height).rev() {
      let r = (i as f64) / ((width - 1) as f64);
      let g = (j as f64) / ((width - 1) as f64);
      let b = 0.25;
      let pixel_color = Color::new(r, g, b);
      write_color(pixel_color);
    }
  }
  eprintln!("\nDone\n");
}
