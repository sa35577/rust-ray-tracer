mod color;
mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;

mod rtweekend;
use rtweekend::{Color, Vec3, Point3, Ray, write_color, INFINITY};
use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;
use sphere::Sphere;
use interval::Interval;
use camera::Camera;

fn main() {

    // World
    let mut world = HittableList::new(Vec::new());
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.render(&world);

}
