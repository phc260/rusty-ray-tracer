extern crate glam;

use glam::{DVec3};

use crate::raytracer::ray::{HitRecord, Ray};
use crate::raytracer::material::{Material};
use std::f64::consts::PI;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant >= 0.0 {
            let t1 = (-half_b - discriminant.sqrt())/a;
            let t2 = (-half_b + discriminant.sqrt())/a;
            for t in [t1, t2].iter() {
                if *t > t_min && *t < t_max {
                    let point = ray.at(*t);
                    let normal = (point-self.center)/self.radius;
                    let front_face = ray.direction.dot(normal) < 0.0;

                    let atan = point.x.atan2(point.z) / PI;
                    let u = atan * 0.5 + 0.5;
                    let v = point.y * 0.5 + 0.5;
                    return Some(
                        HitRecord {
                            t: *t,
                            point: point,
                            normal: if front_face { normal } else { -normal },
                            u: u,
                            v: v,
                            front_face: front_face,
                            material: &self.material,
                        }
                    );
                }
            }
        }
        None
    }
}

pub struct World {
    pub components: Vec<Sphere>,
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut hitrec = None;
        for obj in self.components.iter() {
            if let Some(hit) = obj.hit(ray, t_min, closest) {
                closest = hit.t;
                hitrec = Some(hit);
            }
        }
        hitrec
    }
}
