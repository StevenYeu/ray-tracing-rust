use super::vec3::Point;

#[allow(dead_code)]
pub struct Sphere {
    center: Point,
    radius: f64,
}
#[allow(dead_code)]
impl Sphere {
    pub fn new(cen: Point, r: f64) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
        }
    }
}
