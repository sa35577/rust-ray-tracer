use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::*;
use crate::vec3::*;
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Option<Rc<dyn Material>>) -> Self {
        Self {
            center,
            radius: f64::max(0.0, radius),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        
        // Find the nearest root that lies in the acceptable range
        let mut root = (half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();
        true
    }
}
