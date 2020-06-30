use super::ray;
use super::vec3::Point;
use super::vec3::Vec3;

pub struct hit_record {
  p: Point,
  normla: Vec3,
  t: f64,
}
