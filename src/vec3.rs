
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use crate::utility::{random_float, random_float_range};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub value: [f64; 3],
}

impl Vec3 {
    pub fn default() -> Vec3 {
        Vec3 { value: [0.0, 0.0, 0.0] }
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { value: [e0, e1, e2] }
    }

    pub fn get_value(&self, index: u8) -> f64 {
        self.value[index as usize]
    }

    pub fn get_x(&self) -> f64 {
        self.value[0]
    }

    pub fn get_y(&self) -> f64 {
        self.value[1]
    }

    pub fn get_z(&self) -> f64 {
        self.value[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.value[0] * self.value[0] +
        self.value[1] * self.value[1] +
        self.value[2] * self.value[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.value[0] * other.value[0] +
        self.value[1] * other.value[1] +
        self.value[2] * other.value[2]
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            value: [
                self.value[1] * other.value[2] - self.value[2] * other.value[1],
                self.value[2] * other.value[0] - self.value[0] * other.value[2],
                self.value[0] * other.value[1] - self.value[1] * other.value[0]
            ]
        }
    }

    pub fn random() -> Vec3 {
        Vec3 {
            value: [
                random_float(),
                random_float(),
                random_float()
            ]
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            value: [
                random_float_range(min, max),
                random_float_range(min, max),
                random_float_range(min, max)
            ]
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }

    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.value[0].abs() < s &&
        self.value[1].abs() < s &&
        self.value[2].abs() < s
    }

    pub fn reflect(v: &mut Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * Vec3::dot(*v, *n) * *n
    }

    pub fn print(&self) {
        println!("{} {} {}",
                 self.value[0],
                 self.value[1],
                 self.value[2]
        );
    }
}

pub fn unit_vector(vec: Vec3) -> Vec3 {
    vec / vec.length()
}


impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            value: [
                self.value[0] + other.value[0],
                self.value[1] + other.value[1],
                self.value[2] + other.value[2]
            ]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.value[0] += other.value[0];
        self.value[1] += other.value[1];
        self.value[2] += other.value[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            value: [
                self.value[0] - other.value[0],
                self.value[1] - other.value[1],
                self.value[2] - other.value[2]
            ]
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            value: [
                -self.value[0],
                -self.value[1],
                -self.value[2]
            ]
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            value: [
                self.value[0] * other.value[0],
                self.value[1] * other.value[1],
                self.value[2] * other.value[2]
            ]
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            value: [
                self.value[0] * rhs,
                self.value[1] * rhs,
                self.value[2] * rhs
            ]
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        self * (1.0 / other)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            value: [
                rhs.value[0] * self,
                rhs.value[1] * self,
                rhs.value[2] * self
            ]
        }
    }
}