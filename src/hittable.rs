use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitRecord {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3,
}

pub trait Hittable {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
