mod rtweekend;
mod vec3;
mod ray;
mod color;
mod hittable;
mod hittable_list;
mod sphere;
mod interval;
mod camera;

use rtweekend::Point3;
use hittable_list::HittableList;
use sphere::Sphere;
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
    camera.samples_per_pixel = 100;
    
    camera.render(&world);
}
