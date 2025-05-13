mod color;
mod vec3;
mod ray;

use color::*;
use vec3::*;
use ray::*;

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> bool {
    let oc = center - r.origin();
    // solving equation (C-P) dot (C-P) = r^2
    // becomes (-td + (C-Q)) dot (-td + (C-Q)) = r^2
    // becomes t^2 * d dot d + 2t * (C-Q) dot d + (C-Q) dot (C-Q) - r^2 = 0
    // if discriminant is non-negative, there is a real solution for t and the ray intersects the sphere
    // and then we can colour the pixel red
    let a = r.direction().dot(&r.direction());
    let b = 2.0 * oc.dot(&r.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(r: Ray) -> Color {
    // Color::new(0.0, 0.0, 0.0)
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) { // sphere that is in the center of the world with radius 0.5, check intersection with ray at any point
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    // blue to white gradient
    // uses the (1-a) * white + a * blue
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from the top left corner of the viewport to its bottom right corner
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel
    // negative z moves from camera center to the viewport
    // viewport_u / 2.0 moves from the center to the left edge
    // viewport_v / 2.0 moves from the center to the top edge
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            // let r = Color::new(i as f64 / (image_width - 1) as f64, j as f64 / (image_height - 1) as f64, 0.0);
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(r);
            write_color(&mut std::io::stdout(), pixel_color);
        }
    }
    eprintln!("Done.");
}
