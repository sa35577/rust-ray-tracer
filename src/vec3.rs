use std::ops::Add;
use std::ops::Neg;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::AddAssign;
use crate::rtweekend::{random_double, random_double_range};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;


impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {    
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x()
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max)
        )
    }

    pub fn random() -> Vec3 {
        Vec3::random_range(0.0, 1.0)
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let length_squared = p.length_squared();
            if 1e-160 < length_squared && length_squared <= 1.0 {
                return p / length_squared.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
    
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

fn main() {
    let mut v = Vec3::new(1.0, 2.0, 3.0);
    println!("Original: {} {} {}", v[0], v[1], v[2]);
    
    // Demonstrate mutable indexing
    v[0] = 4.0;
    println!("After modification: {} {} {}", v[0], v[1], v[2]);
    
    let neg_v = -v;
    println!("Negated: {} {} {}", neg_v[0], neg_v[1], neg_v[2]);

    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    let v3 = v1 + v2;
    println!("Addition: {} {} {}", v3[0], v3[1], v3[2]);

    let v4 = v1 * 2.0;
    println!("Multiplication: {} {} {}", v4[0], v4[1], v4[2]);

    let v5 = v1 / 2.0;
    println!("Division: {} {} {}", v5[0], v5[1], v5[2]);

    
}