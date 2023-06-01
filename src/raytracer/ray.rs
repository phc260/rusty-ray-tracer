extern crate glam;

use glam::{DVec3};
use crate::raytracer::material::Material;

pub struct HitRecord<'mat> {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: &'mat Material,
}

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}


impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Ray {
        Self {
            origin: origin,
            direction: direction.normalize(),
        }
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + self.direction*t
    }
}
