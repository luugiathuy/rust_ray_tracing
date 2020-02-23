use super::hittable::HitRecord;
use super::hittable::Hittable;
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
  pub material: Box<dyn Material>,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Self {
    Sphere {
      center,
      radius,
      material,
    }
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
        return Some(HitRecord {
          t,
          p,
          normal,
          material: &*self.material,
        });
      }
      let t = (-b + discriminant.sqrt()) / a;
      if t_min < t && t < t_max {
        let p = r.point_at_parameter(t);
        let normal = (p - self.center) / self.radius;
        return Some(HitRecord {
          t,
          p,
          normal,
          material: &*self.material,
        });
      }
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use super::super::material::Lambertian;
  use super::*;

  #[test]
  fn sphere_not_hit() {
    let r = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0));
    let sphere = Sphere::new(
      Vec3(-3.0, 2.0, 0.0),
      2.0,
      Box::new(Lambertian::new(Vec3(0.3, 0.3, 0.3))),
    );
    assert!(sphere.hit(&r, 0.0001, std::f32::MAX).is_none());
  }

  #[test]
  fn sphere_tangent() {
    let r = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0));
    let sphere = Sphere::new(
      Vec3(2.0, 2.0, 0.0),
      2.0,
      Box::new(Lambertian::new(Vec3(0.3, 0.3, 0.3))),
    );
    assert!(sphere.hit(&r, 0.0001, std::f32::MAX).is_none());
  }

  #[test]
  fn sphere_hit() {
    let r = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0));
    let sphere = Sphere::new(
      Vec3(2.0, 2.0, 0.0),
      3.0,
      Box::new(Lambertian::new(Vec3(0.3, 0.3, 0.3))),
    );
    let rec = sphere.hit(&r, 0.0001, std::f32::MAX);
    assert!(rec.is_some());
    assert_eq!(4.236068, rec.unwrap().t);
    assert_eq!(Vec3(4.236068, 0.0, 0.0), rec.unwrap().p);
    assert_eq!(Vec3(0.74535596, -0.6666667, 0.0), rec.unwrap().normal);
  }
}
