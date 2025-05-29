extern crate rand;
use rand::Rng;

pub use crate::color::{Color, write_color};
pub use crate::vec3::{Vec3, Point3};
pub use crate::ray::Ray;
pub use crate::interval::Interval;
pub const INFINITY: f64 = f64::MAX;
const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    rand::thread_rng().gen_range(min..max)
}