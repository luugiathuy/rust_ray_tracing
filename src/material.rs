use super::hittable::HitRecord;
use super::ray::Ray;
use super::vec3::{random_in_unit_sphere, reflect, refract, Vec3};

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

pub struct Dielectric {
  pub ref_idx: f32,
}

impl Dielectric {
  pub fn new(ref_idx: f32) -> Self {
    Dielectric { ref_idx }
  }
}

impl Material for Dielectric {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
    let attenuation = Vec3(1.0, 1.0, 1.0);
    let (outward_normal, ni_over_nt, cosine) = if r_in.direction.dot(rec.normal) > 0.0 {
      let cosine = self.ref_idx * r_in.direction.dot(rec.normal) / r_in.direction.length();
      (-rec.normal, self.ref_idx, cosine)
    } else {
      let cosine = -r_in.direction.dot(rec.normal) / r_in.direction.length();
      (rec.normal, 1.0 / self.ref_idx, cosine)
    };
    let reflected = Ray::new(rec.p, reflect(r_in.direction, rec.normal));
    let scattered = match refract(r_in.direction, outward_normal, ni_over_nt) {
      Some(refracted_dir) => {
        let reflect_prob = schlick(cosine, self.ref_idx);
        if rand::random::<f32>() < reflect_prob {
          reflected
        } else {
          Ray::new(rec.p, refracted_dir)
        }
      }
      None => reflected,
    };
    Some(Scatter {
      scattered,
      attenuation,
    })
  }
}

/// Christophe Schlick's approximation is a formula for approximating the contribution of the Fresnel factor in the specular reflection of light from a non-conducting interface
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
  let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
  let r0 = r0 * r0;
  r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
