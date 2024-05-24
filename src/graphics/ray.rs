use super::vec3::Point;
use super::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Point,
    pub dir: Vec3,
}
impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point {
        return self.orig;
    }

    pub fn direction(&self) -> Vec3 {
        return self.dir;
    }

    pub fn at(&self, t: f64) -> Point {
        return self.orig + t * self.dir;
    }
}
