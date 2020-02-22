extern crate rand;
use rand::Rng;

mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn color(r: &Ray, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
    match world.hit(r, 0.001, std::f32::MAX) {
        Some(rec) => {
            let target = rec.p + rec.normal + vec3::random_in_unit_sphere();
            0.5 * color(&Ray::new(rec.p, target - rec.p), world)
        }
        None => {
            let unit_direction = r.direction.to_unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let cam = Camera::new();
    let mut rng = rand::thread_rng();

    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    world.push(Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)));
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &world);
            }
            col /= ns as f32;
            col = Vec3(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
