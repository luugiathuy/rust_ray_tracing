use super::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
  pub origin: Vec3,
  pub direction: Vec3,
}

impl Ray {
  pub fn new(o: Vec3, d: Vec3) -> Self {
    Ray {
      origin: o,
      direction: d,
    }
  }

  pub fn point_at_parameter(&self, t: f32) -> Vec3 {
    self.origin + t * self.direction
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn point_at_parameter() {
    let o = Vec3(1.0, 2.0, 3.0);
    let d = Vec3(1.0, 1.0, 0.0);
    let r = Ray::new(o, d);
    let p = r.point_at_parameter(5.0);
    assert_eq!(6.0, p.x());
    assert_eq!(7.0, p.y());
    assert_eq!(3.0, p.z());
  }
}
