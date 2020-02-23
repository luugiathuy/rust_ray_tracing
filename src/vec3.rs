extern crate rand;
use rand::Rng;

use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
  // x, y, z
  pub fn x(&self) -> f32 {
    self.0
  }
  pub fn y(&self) -> f32 {
    self.1
  }
  pub fn z(&self) -> f32 {
    self.2
  }

  // r, g, b
  pub fn r(&self) -> f32 {
    self.0
  }
  pub fn g(&self) -> f32 {
    self.1
  }
  pub fn b(&self) -> f32 {
    self.2
  }

  pub fn dot(&self, other: Vec3) -> f32 {
    self.0 * other.0 + self.1 * other.1 + self.2 * other.2
  }

  pub fn cross(&self, other: Vec3) -> Vec3 {
    Vec3(
      self.1 * other.2 - self.2 * other.1,
      -(self.0 * other.2 - self.2 * other.0),
      self.0 * other.1 - self.1 * other.0,
    )
  }

  pub fn squared_length(self) -> f32 {
    self.dot(self)
  }

  pub fn length(self) -> f32 {
    self.squared_length().sqrt()
  }

  pub fn to_unit_vector(&self) -> Vec3 {
    *self / self.length()
  }
}

impl Neg for Vec3 {
  type Output = Vec3;
  fn neg(self) -> Vec3 {
    Vec3(-self.0, -self.1, -self.2)
  }
}

impl Add for Vec3 {
  type Output = Vec3;
  fn add(self, other: Vec3) -> Vec3 {
    Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
  }
}

impl Sub for Vec3 {
  type Output = Vec3;
  fn sub(self, other: Vec3) -> Vec3 {
    Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
  }
}

// v1 * v2 element wise multiplication
impl Mul<Vec3> for Vec3 {
  type Output = Vec3;
  fn mul(self, v: Vec3) -> Vec3 {
    Vec3(self.0 * v.0, self.1 * v.1, self.2 * v.2)
  }
}

// vec * float
impl Mul<f32> for Vec3 {
  type Output = Vec3;
  fn mul(self, t: f32) -> Vec3 {
    Vec3(self.0 * t, self.1 * t, self.2 * t)
  }
}

// float * vec
impl Mul<Vec3> for f32 {
  type Output = Vec3;
  fn mul(self, v: Vec3) -> Vec3 {
    Vec3(self * v.0, self * v.1, self * v.2)
  }
}

impl Div<Vec3> for Vec3 {
  type Output = Vec3;
  fn div(self, v: Vec3) -> Vec3 {
    Vec3(self.0 / v.0, self.1 / v.1, self.2 / v.2)
  }
}

impl Div<f32> for Vec3 {
  type Output = Vec3;
  fn div(self, t: f32) -> Vec3 {
    self * (1.0 / t)
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, other: Vec3) {
    self.0 += other.0;
    self.1 += other.1;
    self.2 += other.2;
  }
}

impl SubAssign for Vec3 {
  fn sub_assign(&mut self, other: Vec3) {
    self.0 -= other.0;
    self.1 -= other.1;
    self.2 -= other.2;
  }
}

impl MulAssign<Vec3> for Vec3 {
  fn mul_assign(&mut self, other: Vec3) {
    self.0 *= other.0;
    self.1 *= other.1;
    self.2 *= other.2;
  }
}

impl MulAssign<f32> for Vec3 {
  fn mul_assign(&mut self, t: f32) {
    self.0 *= t;
    self.1 *= t;
    self.2 *= t;
  }
}

impl DivAssign<Vec3> for Vec3 {
  fn div_assign(&mut self, other: Vec3) {
    self.0 /= other.0;
    self.1 /= other.1;
    self.2 /= other.2;
  }
}

impl DivAssign<f32> for Vec3 {
  fn div_assign(&mut self, t: f32) {
    *self *= 1.0 / t;
  }
}

pub fn random_in_unit_sphere() -> Vec3 {
  let mut rng = rand::thread_rng();
  let unit = Vec3(1.0, 1.0, 1.0);
  loop {
    let p = 2.0 * Vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - unit;
    if p.squared_length() < 1.0 {
      return p;
    }
  }
}

pub fn random_in_unit_disk() -> Vec3 {
  let mut rng = rand::thread_rng();
  loop {
    let p = Vec3(
      2.0 * rng.gen::<f32>() - 1.0,
      2.0 * rng.gen::<f32>() - 1.0,
      0.0,
    );
    if p.squared_length() < 1.0 {
      return p;
    }
  }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
  v - 2.0 * v.dot(n) * n
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
  let uv = v.to_unit_vector();
  let dt = uv.dot(n);
  let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
  if discriminant > 0.0 {
    Some(ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n)
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dot_vec() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(4.0, 5.0, 6.0);
    assert_eq!(32.0, v1.dot(v2));
  }

  #[test]
  fn cross_product() {
    let v1 = Vec3(2.0, 3.0, 4.0);
    let v2 = Vec3(5.0, 6.0, 7.0);
    let v3 = v1.cross(v2);
    assert_eq!(-3.0, v3.x());
    assert_eq!(6.0, v3.y());
    assert_eq!(-3.0, v3.z());
  }

  #[test]
  fn vec_squared_length() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    assert_eq!(14.0, v1.squared_length());
  }

  #[test]
  fn vec_length() {
    let v1 = Vec3(2.0, 3.0, 4.0);
    assert_eq!((29.0 as f32).sqrt(), v1.length())
  }

  #[test]
  fn unit_vector() {
    let v1 = Vec3(2.0, 4.0, 4.0);
    let v2 = v1.to_unit_vector();
    assert_eq!(1.0 / 3.0, v2.x());
    assert_eq!(2.0 / 3.0, v2.y());
    assert_eq!(2.0 / 3.0, v2.z());
  }

  #[test]
  fn neg_vec() {
    let v = -Vec3(1.0, 2.0, 3.0);
    assert_eq!(-1.0, v.x());
    assert_eq!(-2.0, v.y());
    assert_eq!(-3.0, v.z());
  }

  #[test]
  fn add_vec() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(4.0, 5.0, 6.0);
    let mut v3 = v1 + v2;
    assert_eq!(5.0, v3.r());
    assert_eq!(7.0, v3.g());
    assert_eq!(9.0, v3.b());
    v3 += v1;
    assert_eq!(6.0, v3.r());
    assert_eq!(9.0, v3.g());
    assert_eq!(12.0, v3.b());
  }

  #[test]
  fn sub_vec() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(6.0, 5.0, 4.0);
    let mut v3 = v2 - v1;
    assert_eq!(5.0, v3.0);
    assert_eq!(3.0, v3.1);
    assert_eq!(1.0, v3.2);
    v3 -= v1;
    assert_eq!(4.0, v3.0);
    assert_eq!(1.0, v3.1);
    assert_eq!(-2.0, v3.2);
  }

  #[test]
  fn mul_vec() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(4.0, 5.0, 6.0);
    let mut v3 = v1 * v2;
    assert_eq!(4.0, v3.x());
    assert_eq!(10.0, v3.y());
    assert_eq!(18.0, v3.z());
    v3 *= v1;
    assert_eq!(4.0, v3.x());
    assert_eq!(20.0, v3.y());
    assert_eq!(54.0, v3.z());
  }

  #[test]
  fn mul_vec_with_number() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let mut v2 = 2.0 * v1 * 5.0;
    assert_eq!(10.0, v2.x());
    assert_eq!(20.0, v2.y());
    assert_eq!(30.0, v2.z());
    v2 *= 3.0;
    assert_eq!(30.0, v2.x());
    assert_eq!(60.0, v2.y());
    assert_eq!(90.0, v2.z());
  }

  #[test]
  fn div_vec() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(2.0, 6.0, 12.0);
    let mut v3 = v2 / v1;
    assert_eq!(2.0, v3.x());
    assert_eq!(3.0, v3.y());
    assert_eq!(4.0, v3.z());
    v3 /= v1;
    assert_eq!(2.0, v3.x());
    assert_eq!(1.5, v3.y());
    assert_eq!(4.0 / 3.0, v3.z());
  }

  #[test]
  fn div_vec_by_number() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let mut v2 = v1 / 2.0;
    assert_eq!(0.5, v2.x());
    assert_eq!(1.0, v2.y());
    assert_eq!(1.5, v2.z());
    v2 /= 2.0;
    assert_eq!(0.25, v2.x());
    assert_eq!(0.5, v2.y());
    assert_eq!(0.75, v2.z());
  }

  #[test]
  fn reflect_vec() {
    let v = Vec3(-1.0, -1.0, 0.0);
    let n = Vec3(0.0, 1.0, 0.0);
    let r = reflect(v, n);
    assert_eq!(Vec3(-1.0, 1.0, 0.0), r);
  }
}
