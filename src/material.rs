use super::hittable::HitRecord;
use super::ray::Ray;
use super::vec3::{random_in_unit_sphere, reflect, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Scatter {
  pub scattered: Ray,
  pub attenuation: Vec3,
}

pub trait Material {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
  pub albedo: Vec3,
}

impl Lambertian {
  pub fn new(albedo: Vec3) -> Self {
    Lambertian { albedo }
  }
}

impl Material for Lambertian {
  fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
    let target = rec.p + rec.normal + random_in_unit_sphere();
    let scattered = Ray::new(rec.p, target - rec.p);
    let attenuation = self.albedo;
    Some(Scatter {
      scattered,
      attenuation,
    })
  }
}

pub struct Metal {
  pub albedo: Vec3,
  pub fuzz: f32,
}

impl Metal {
  pub fn new(albedo: Vec3, fuzz: f32) -> Self {
    Metal {
      albedo,
      fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
    }
  }
}

impl Material for Metal {
  fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
    let reflected = reflect(ray.direction.to_unit_vector(), rec.normal);
    let reflected_fuzzed = reflected + self.fuzz * random_in_unit_sphere();
    if reflected_fuzzed.dot(rec.normal) > 0.0 {
      let scattered = Ray::new(rec.p, reflected_fuzzed);
      let attenuation = self.albedo;
      Some(Scatter {
        scattered,
        attenuation,
      })
    } else {
      None
    }
  }
}
