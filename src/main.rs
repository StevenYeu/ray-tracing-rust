mod graphics;
use graphics::color::write_color;
use graphics::ray::Ray;
use graphics::vec3::Color;
use graphics::vec3::Point;
use graphics::vec3::Vec3;

fn hit_sphere(center: Point, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = Vec3::dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminat = (half_b * half_b) - (a * c);
    if discriminat < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminat.sqrt()) / a;
    }
}

fn hit_cub(center: Point, length: f64, r: Ray) -> f64 {

}

fn color_sphere(r: Ray) -> Color {
    let t = hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, r);

    // Color Shape
    if t > 0.0 {
        let n = Vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    // Background Gradient Direction
    let tb = 0.5 * (unit_direction.x() + 1.0);
    // Baclground Color
    return ((1.0 - tb) * Color::new(0.922, 0.435, 0.573)) + (tb * Color::new(0.192, 0.455, 0.561));
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i64 = 1920;
    let image_height: i64 = (image_width as f64 / aspect_ratio) as i64;
    println!("P3\n{} {}\n255", image_width, image_height);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_legth = 1.0;

    // Position of Camera/Viewport
    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let veritcal = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (veritcal / 2.0) - Vec3::new(0.0, 0.0, focal_legth);

    for j in (0..(image_height)).rev() {
        eprintln!("\rScanlines reamning: {} ", j);
        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * veritcal - origin,
            );
            let pixel_color = color_sphere(r);
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone\n");
}
