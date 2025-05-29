use crate::vec3::Vec3;
use crate::rtweekend::*;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component <= 0.0 {
        0.0
    } else {
        linear_component.sqrt()
    }
}

pub fn write_color(out: &mut dyn std::io::Write, pixel_color: Color) {
    let mut r = linear_to_gamma(pixel_color.x());
    let mut g = linear_to_gamma(pixel_color.y());
    let mut b = linear_to_gamma(pixel_color.z());

    // Translate the [0,1] component values to the byte range [0,255]
    let intensity = Interval::new(0.0, 0.999);
    let ir = (255.999 * intensity.clamp(r)) as i32;
    let ig = (255.999 * intensity.clamp(g)) as i32;
    let ib = (255.999 * intensity.clamp(b)) as i32;

    out.write_fmt(format_args!("{} {} {}\n", ir, ig, ib));
}
