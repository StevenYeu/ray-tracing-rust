use super::hittable;
use super::vec3::Point;

pub struct Sphere {
  center: Point,
  radius: f64,
}

impl Sphere {
  pub fn new(cen: Point, r: f64) -> Sphere {
    Sphere {
      center: cen,
      radius: r,
    }
  }
}
