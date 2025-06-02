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

use rtweekend::{Point3, Vec3, PI, random_double, random_double_range};
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};
use color::Color;
use std::rc::Rc;

fn main() {
    // World
    let mut world = HittableList::new(Vec::new());

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Some(ground_material.clone()))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, Some(sphere_material.clone()))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, Some(sphere_material.clone()))));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, Some(sphere_material.clone()))));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Some(material1.clone()))));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Some(material2.clone()))));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Some(material3.clone()))));


    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    


    // let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let material_left = Rc::new(Dielectric::new(1.5));
    // let material_bubble = Rc::new(Dielectric::new(1.00 / 1.50));
    // let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Some(material_ground.clone()))));
    // world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, Some(material_center.clone()))));
    // world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Some(material_left.clone()))));
    // world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, Some(material_bubble.clone()))));
    // world.add(Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Some(material_right.clone()))));

    // // Camera
    // let mut camera = Camera::new();
    // camera.aspect_ratio = 16.0 / 9.0;
    // camera.image_width = 400;
    // camera.samples_per_pixel = 100;
    // camera.max_depth = 50;
    // camera.vfov = 20.0;
    // camera.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    // camera.lookat = Point3::new(0.0, 0.0, -1.0);
    // camera.vup = Vec3::new(0.0, 1.0, 0.0);
    // camera.defocus_angle = 10.0;
    // camera.focus_dist = 3.4;
    
    camera.render(&world);
}
