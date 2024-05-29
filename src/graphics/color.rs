use super::vec3::Color;
pub fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (255.0 * pixel_color.x()).floor() as i64,
        (255.0 * pixel_color.y()).floor() as i64,
        (255.0 * pixel_color.z()).floor() as i64
    )
    
}
