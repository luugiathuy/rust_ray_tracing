use super::ray::Ray;
use super::vec3::{random_in_unit_disk, Vec3};
use std::f32;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
  origin: Vec3,
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
  u: Vec3,
  v: Vec3,
  w: Vec3,
  lens_radius: f32,
}

impl Camera {
  pub fn new(
    look_from: Vec3,
    look_at: Vec3,
    view_up: Vec3,
    vertical_fov: f32, // vertical_fov is top to bottom in degrees
    aspect: f32,
    aperture: f32,
    focus_dist: f32,
  ) -> Self {
    let theta = vertical_fov * f32::consts::PI / 180.0;
    let half_height = f32::tan(theta * 0.5);
    let half_width = aspect * half_height;
    let w = (look_from - look_at).to_unit_vector();
    let u = view_up.cross(w).to_unit_vector();
    let v = w.cross(u);
    Camera {
      origin: look_from,
      lower_left_corner: look_from
        - half_width * focus_dist * u
        - half_height * focus_dist * v
        - focus_dist * w,
      horizontal: 2.0 * half_width * focus_dist * u,
      vertical: 2.0 * half_height * focus_dist * v,
      u,
      v,
      w,
      lens_radius: aperture * 0.5,
    }
  }

  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    let rd = self.lens_radius * random_in_unit_disk();
    let offset = self.u * rd.x() + self.v * rd.y();
    Ray::new(
      self.origin + offset,
      self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
    )
  }
}
