use super::ray::Ray;
use super::vec3::Vec3;
use std::f32;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
  origin: Vec3,
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
}

impl Camera {
  pub fn new(
    look_from: Vec3,
    look_at: Vec3,
    view_up: Vec3,
    vertical_fov: f32,
    aspect: f32,
  ) -> Self {
    // vertical_fov is top to bottom in degrees
    let theta = vertical_fov * f32::consts::PI / 180.0;
    let half_height = f32::tan(theta * 0.5);
    let half_width = aspect * half_height;
    let w = (look_from - look_at).to_unit_vector();
    let u = view_up.cross(w).to_unit_vector();
    let v = w.cross(u);
    Camera {
      origin: look_from,
      lower_left_corner: look_from - half_width * u - half_height * v - w,
      horizontal: 2.0 * half_width * u,
      vertical: 2.0 * half_height * v,
    }
  }

  pub fn get_ray(&self, u: f32, v: f32) -> Ray {
    Ray::new(
      self.origin,
      self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
    )
  }
}
