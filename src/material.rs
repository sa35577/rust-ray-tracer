use crate::rtweekend::*;
use crate::hittable::*;
use crate::ray::*;
use crate::color::*;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
    // fn clone_box(&self) -> Box<dyn Material>; //???
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        // eprintln!("Lambertian::scatter");
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }

    // fn clone_box(&self) -> Box<dyn Material> {
    //     Box::new(self.clone())
    // }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(a: Color) -> Self {
        Self { albedo: a }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        // eprintln!("Metal::scatter");
        let reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }

    // fn clone_box(&self) -> Box<dyn Material> {
    //     Box::new(self.clone())
    // }
}


