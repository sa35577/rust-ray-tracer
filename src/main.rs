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

use rtweekend::{Point3, Vec3, PI};
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};
use color::Color;
use std::rc::Rc;

fn main() {
    // World
    let mut world = HittableList::new(Vec::new());
    // world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    // world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));


    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1.00 / 1.50));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Some(material_ground.clone()))));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, Some(material_center.clone()))));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Some(material_left.clone()))));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, Some(material_bubble.clone()))));
    world.add(Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Some(material_right.clone()))));

    // let R = (PI / 4.0).cos();
    // let material_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    // let material_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    // world.add(Rc::new(Sphere::new(Point3::new(-R, 0.0, -1.0), R, Some(material_left.clone()))));
    // world.add(Rc::new(Sphere::new(Point3::new(R, 0.0, -1.0), R, Some(material_right.clone()))));

    // Camera
    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    camera.lookat = Point3::new(0.0, 0.0, -1.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 10.0;
    camera.focus_dist = 3.4;
    
    camera.render(&world);
}
