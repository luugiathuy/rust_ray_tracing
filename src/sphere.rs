use super::hittable::HitRecord;
use super::hittable::Hittable;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32) -> Self {
    Sphere { center, radius }
  }
}

impl Hittable for Sphere {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let oc = r.origin - self.center;
    let a = r.direction.dot(r.direction);
    let b = oc.dot(r.direction);
    let c = oc.dot(oc) - self.radius * self.radius;
    let discriminant = b * b - a * c;
    if discriminant > 0.0 {
      let t = (-b - discriminant.sqrt()) / a;
      if t_min < t && t < t_max {
        let p = r.point_at_parameter(t);
        let normal = (p - self.center) / self.radius;
        return Some(HitRecord { t, p, normal });
      }
      let t = (-b + discriminant.sqrt()) / a;
      if t_min < t && t < t_max {
        let p = r.point_at_parameter(t);
        let normal = (p - self.center) / self.radius;
        return Some(HitRecord { t, p, normal });
      }
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sphere_not_hit() {
    let r = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0));
    let sphere = Sphere::new(Vec3(-3.0, 2.0, 0.0), 2.0);
    assert_eq!(None, sphere.hit(&r, 0.0001, std::f32::MAX));
  }

  #[test]
  fn sphere_tangent() {
    let r = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0));
    let sphere = Sphere::new(Vec3(2.0, 2.0, 0.0), 2.0);
    assert_eq!(None, sphere.hit(&r, 0.0001, std::f32::MAX));
  }

  #[test]
  fn sphere_hit() {
    let r = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0));
    let sphere = Sphere::new(Vec3(2.0, 2.0, 0.0), 3.0);
    assert_eq!(
      Some(HitRecord {
        t: 4.236068,
        p: Vec3(4.236068, 0.0, 0.0),
        normal: Vec3(0.74535596, -0.6666667, 0.0)
      }),
      sphere.hit(&r, 0.0001, std::f32::MAX)
    );
  }
}
