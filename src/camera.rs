use crate::rtweekend::{Color, Vec3, Point3, Ray, write_color, INFINITY};
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use rand::Rng;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pixel_samples_scale: f64,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            pixel_samples_scale: 1.0,
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    fn initialize(&mut self) {
        // Calculate the image height, and ensure that it's at least 1
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);
        eprintln!("pixel_samples_scale: {}", self.pixel_samples_scale);

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.center = Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0, false);
        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
            let mut attenuation = Color::new(0.0, 0.0, 0.0);
            if let Some(mat) = &rec.mat {
                if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * self.ray_color(&scattered, depth - 1, world);
                }
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        // blue to white gradient
        // uses the (1-a) * white + a * blue
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    fn sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let x = -0.5 + rng.gen_range(0.0..1.0);
        let y = -0.5 + rng.gen_range(0.0..1.0);
        // eprintln!("x: {}, y: {}", x, y);
        Vec3::new(x, y, 0.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc + 
            ((i as f64 + offset.x()) * self.pixel_delta_u) + 
            ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_direction = pixel_sample - self.center;
        Ray::new(self.center, ray_direction)
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        // Render
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {}", self.image_height - j);
            // eprintln!("samples_per_pixel: {}", self.samples_per_pixel);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }

                let result = self.pixel_samples_scale * pixel_color;
                // eprintln!("result: x: {}, y: {}, z: {}", result.x(), result.y(), result.z());
                write_color(&mut std::io::stdout(), result);
            }
        }
        eprintln!("Done.");
    }
}