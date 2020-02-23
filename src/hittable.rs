use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct HitRecord<'obj> {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3,
  pub material: &'obj dyn Material,
}

pub trait Hittable {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
