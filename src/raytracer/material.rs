extern crate glam;

use glam::{DVec3};

type Color3=DVec3;

use crate::raytracer::ray::{HitRecord, Ray};
use rand::Rng;

use std::f64::consts::PI;

fn rand_in_unit_sphere() -> DVec3 {
    let mut rng = rand::thread_rng();
    let radius: f64 = rng.gen_range(0.0..1.0);
    let theta: f64 = rng.gen_range(0.0..PI);
    let phi: f64 = rng.gen_range(-PI..PI);
    DVec3::new(
        radius * theta.sin() * phi.cos(),
        radius * theta.sin() * phi.sin(),
        radius * theta.cos(),
    )
}

fn rand_unit_vector() -> DVec3 {
    let mut rng = rand::thread_rng();
    let theta: f64 = rng.gen_range(0.0..PI);
    let phi: f64 = rng.gen_range(-PI..PI);
    DVec3::new(
       theta.sin() * phi.cos(),
       theta.sin() * phi.sin(),
       theta.cos(), 
    )
}

fn reflect(v: &DVec3, n: &DVec3) -> DVec3 {
    *v - *n * (2.0 * v.dot(*n))
}

fn refract(uv: &DVec3, n: &DVec3, etai_over_etat: f64) -> DVec3 {
    let cos = (-*uv).dot(*n);
    let r_perp = (*uv + *n * cos) * etai_over_etat;
    let r_parallel = -(1.0 - r_perp.length_squared()).abs().sqrt() * (*n);
    r_perp + r_parallel
}

fn reflectance(cos: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 *= r0;
    r0 + (1.0-r0) * ((1.0-cos).powi(5))
}

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hitrec: &HitRecord) -> (Ray, Color3);
}

pub struct Lambertian {
    pub albedo: Color3,
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hitrec: &HitRecord) -> (Ray, Color3) {
        fn near_zero(v: &DVec3) -> bool {
            v.x < f64::EPSILON && v.y < f64::EPSILON && v.z < f64::EPSILON
        }
        let mut direction = hitrec.normal + rand_unit_vector();
        if near_zero(&direction) {
            direction = hitrec.normal;
        }
        let scattered = Ray::new(hitrec.point, direction);
        return (scattered, self.albedo);
    }
}

pub struct Metal {
    pub albedo: Color3,
    pub fuzz: f64,
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hitrec: &HitRecord) -> (Ray, Color3) {
        let reflected = reflect(&ray.direction, &hitrec.normal);
        let scattered = Ray::new(
                            hitrec.point,
                            reflected+self.fuzz*rand_in_unit_sphere());
        return (scattered, self.albedo);
    }
}

pub struct Dieletric {
    pub albedo: Color3,
    pub index_of_refraction: f64,
}

impl Scatterable for Dieletric {
    fn scatter(&self, ray: &Ray, hitrec: &HitRecord) -> (Ray, Color3) {

        let refraction_ratio = if hitrec.front_face {
            1.0/self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let cos = (-ray.direction).dot(hitrec.normal);
        let sin = (1.0 - cos*cos).sqrt();

        let mut cannot_refract = refraction_ratio * sin > 1.0;
        cannot_refract |= reflectance(cos, refraction_ratio) > rand::random::<f64>();
        let direction = if cannot_refract {
            reflect(&ray.direction, &hitrec.normal)
        } else {
            refract(&ray.direction, &hitrec.normal, refraction_ratio)
        };
        let scattered = Ray::new(hitrec.point, direction);
        (scattered, self.albedo)
    }
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dieletric(Dieletric),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hitrec: &HitRecord) -> (Ray, Color3) {
        match self {
            Material::Lambertian(m) => m.scatter(ray, hitrec),
            Material::Metal(m) => m.scatter(ray, hitrec),
            Material::Dieletric(m) => m.scatter(ray, hitrec),
        }
    }
}
