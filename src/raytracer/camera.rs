extern crate glam;

use crate::raytracer::ray::Ray;
use glam::{DVec3};
use rand::Rng;

fn random_in_unit_disk() -> DVec3 {
    let mut rng = rand::thread_rng();
    let radius = rng.gen_range(0.0..1.0);
    let theta: f64 = rng.gen_range(-std::f64::consts::PI..std::f64::consts::PI);
    DVec3::new(radius * theta.sin(), radius * theta.cos(), 0.0)
}

pub struct Camera {
    origin: DVec3,
    lower_left_corner: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
    u: DVec3,
    v: DVec3,
    lens_radius: f64,
    // time0: f64,
    // time1: f64,
}

impl Camera {
    pub fn new(lookfrom: DVec3, lookat: DVec3, vup:DVec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Self {
        let theta = vfov * 0.01745329251994329576923690768489; // deg to radian
        let viewport_height = 2.0 * (theta*0.5).tan();
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = (lookfrom - lookat).normalize();
        let u = (vup.cross(w)).normalize();
        let v = w.cross(u).normalize();

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = lookfrom - horizontal*0.5 - vertical*0.5 - focus_dist*w;

        Self {
            origin: lookfrom,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            lens_radius: aperture*0.5,
            // time0: 0.0,
            // time1: 0.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin,
            self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset,
        )
    } 
}
