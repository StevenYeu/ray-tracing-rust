use super::ray::Ray;
use super::vec3::Point;
use super::vec3::Vec3;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct HitRecord {
  p: Point,
  norml: Vec3,
  t: f64,
}

pub trait Hittable {
  fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool;
}
