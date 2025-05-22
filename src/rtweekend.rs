pub use color::{Color, write_color};
pub use vec3::{Vec3, Point3};
pub use ray::Ray;

pub const INFINITY: f64 = f64::MAX;
const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
