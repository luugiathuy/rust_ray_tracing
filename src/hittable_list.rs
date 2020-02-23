use super::hittable::HitRecord;
use super::hittable::Hittable;
use super::ray::Ray;

impl Hittable for Vec<Box<dyn Hittable>> {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut best = None;
    for child in self {
      if let Some(hit) = child.hit(r, t_min, t_max) {
        match best {
          None => best = Some(hit),
          Some(prev) => {
            if hit.t < prev.t {
              best = Some(hit)
            }
          }
        }
      }
    }
    best
  }
}

#[cfg(test)]
mod tests {
  use super::super::material::Lambertian;
  use super::super::sphere::Sphere;
  use super::super::vec3::Vec3;
  use super::*;

  #[test]
  fn nearest_hit() {
    let r = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0));
    let spheres: Vec<Sphere> = vec![
      Sphere::new(
        Vec3(8.0, 3.0, 1.0),
        4.0,
        Box::new(Lambertian::new(Vec3(0.3, 0.3, 0.3))),
      ),
      Sphere::new(
        Vec3(2.0, 2.0, 0.0),
        3.0,
        Box::new(Lambertian::new(Vec3(0.3, 0.3, 0.3))),
      ),
    ];
    let list: Vec<Box<dyn Hittable>> = spheres
      .into_iter()
      .map(|s| Box::new(s) as Box<dyn Hittable>)
      .collect();
    let rec = list.hit(&r, 0.0001, std::f32::MAX);
    assert!(rec.is_some());
    assert_eq!(4.236068, rec.unwrap().t);
    assert_eq!(Vec3(4.236068, 0.0, 0.0), rec.unwrap().p);
    assert_eq!(Vec3(0.74535596, -0.6666667, 0.0), rec.unwrap().normal);
  }
}
