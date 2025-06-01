mod rtweekend;
mod vec3;
mod ray;
mod color;
mod hittable;
mod hittable_list;
mod sphere;
mod interval;
mod camera;
mod material;

use rtweekend::Point3;
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;
use material::{Material, Lambertian, Metal};
use color::Color;
use std::rc::Rc;

fn main() {
    // World
    let mut world = HittableList::new(Vec::new());
    // world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    // world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));


    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Some(material_ground.clone()))));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, Some(material_center.clone()))));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Some(material_left.clone()))));
    world.add(Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Some(material_right.clone()))));

    // Camera
    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    
    camera.render(&world);
}
