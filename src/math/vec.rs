use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    pub fn norm(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(*self)
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn abs_dot(&self, other: Vec3) -> f64 {
        self.dot(other).abs()
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        return Self {
            x: difference_of_products(self.y, other.z, self.z, other.y),
            y: difference_of_products(self.z, other.x, self.x, other.z),
            z: difference_of_products(self.x, other.y, self.y, other.x),
        };
    }
}

fn difference_of_products(a: f64, b: f64, c: f64, d: f64) -> f64 {
    let cd = c * d;
    let dop = a * b - cd;
    let error = -c * d + cd;
    return dop + error;
}

pub fn lerp(t: f64, a: Vec3, b: Vec3) -> Vec3 {
    (1. - t) * a + t * b
}

pub fn fma(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    a * b + c
}

pub fn angle_between(a: Vec3, b: Vec3) -> f64 {
    let cos_theta: f64 = a.dot(b) / (a.length() * b.length());
    let theta = cos_theta.acos();
    return theta;
}

pub fn gram_schmidt(v: Vec3, w: Vec3) -> Vec3 {
    // Computes vector orthogonal to w that spans the same subspace as v and w
    let w_norm = w.norm();
    return v - v.dot(w_norm) * w_norm;
}

pub fn distance(a: Point3, b: Point3) {
    (a - b).length();
}

pub fn distance_squared(a: Point3, b: Point3) {
    (a - b).length_squared();
}

pub fn faceforward(n: Vec3, v: Vec3) -> Vec3 {
    if n.dot(v) < 0. {
        return -n;
    } else {
        return n;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec3: Vec3) -> Vec3 {
        Vec3 {
            x: self * vec3.x,
            y: self * vec3.y,
            z: self * vec3.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_product() {
        let a = Vec3::new(1., 0., 0.);
        let b = Vec3::new(0., 1., 0.);
        let c = a.cross(b);

        assert_eq!(c, Vec3::new(0., 0., 1.));
    }
}
