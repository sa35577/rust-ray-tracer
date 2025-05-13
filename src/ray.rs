use crate::vec3::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {

    pub fn new(o: Point3, d: Vec3) -> Ray {
        Ray { origin: o, direction: d }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
    
}