mod raytracer;

extern crate image;
extern crate glam;

use glam::{DVec3};

type Color3 = DVec3;

use crate::raytracer::camera::Camera;
use crate::raytracer::material::{Lambertian, Metal, Dieletric, Material, Scatterable};
use crate::raytracer::ray::{Ray};
use crate::raytracer::shape::{Hittable, Sphere, World};

use indicatif::{ProgressBar, ProgressStyle};
use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;


fn ray_color(ray: &Ray, world: &World, depth: u16) -> Color3 {
    if depth <= 0 {
        return Color3::new(0.0, 0.0, 0.0);
    }

    if let Some(hitrec) = world.hit(ray, 0.001, std::f64::INFINITY) {
        let (scattered, attenuation) = hitrec.material.scatter(ray, &hitrec);
        return attenuation * ray_color(&scattered, world, depth-1);
    }
    
    let t = 0.5*(ray.direction.y + 1.0);
    return (1.0-t)*Color3::new(1.0, 1.0, 1.0) + t*Color3::new(0.5, 0.7, 1.0);
}

fn triplets () -> World {
    World {
        components: vec![
            Sphere {
                center: DVec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Material::Lambertian(
                    Lambertian {
                        albedo: Color3::new(0.8, 0.8, 0.0),
                    },
                ),
            },
            Sphere {
                center: DVec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Lambertian(
                    Lambertian {
                        albedo: Color3::new(0.1, 0.2, 0.5),
                    },
                ),
            },
            Sphere {
                center: DVec3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Dieletric(
                    Dieletric {
                        albedo: Color3::new(1.0, 1.0, 1.0),
                        index_of_refraction: 1.5,
                    },
                ),
            },
            Sphere {
                center: DVec3::new(-1.0, 0.0, -1.0),
                radius: -0.4,
                material: Material::Dieletric(
                    Dieletric {
                        albedo: Color3::new(1.0, 1.0, 1.0),
                        index_of_refraction: 1.5,
                    },
                ),
            },
            Sphere {
                center: DVec3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Metal(
                    Metal {
                        albedo: Color3::new(0.8, 0.6, 0.2),
                        fuzz: 1.0,
                    }
                ),
            },
        ],
    }
}

fn random_scene() -> World {

    let mut world = World {
        components: vec![
            Sphere {
                center: DVec3::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
                material: Material::Lambertian(
                    Lambertian {
                        albedo: Color3::new(0.5, 0.5, 0.5),
                    },
                ),
            },
            Sphere {
                center: DVec3::new(0.0, 1.0, 0.0),
                radius: 1.0,
                material: Material::Dieletric(
                    Dieletric {
                        albedo: Color3::new(1.0, 1.0, 1.0),
                        index_of_refraction: 1.5,
                    },
                ),
            },
            Sphere {
                center: DVec3::new(-4.0, 1.0, 0.0),
                radius: 1.0,
                material: Material::Lambertian(
                    Lambertian {
                        albedo: Color3::new(0.4, 0.2, 0.1),
                    },
                ),
            },
            Sphere {
                center: DVec3::new(1.0, 1.0, 0.0),
                radius: 1.0,
                material: Material::Metal(
                    Metal {
                        albedo: Color3::new(0.7, 0.6, 0.5),
                        fuzz: 0.0,
                    }
                ),
            },
        ],
    };
    
    for _i in -11..11 {
        for _j in -11..11 {
            
        }
    }

    return world;
}

fn main() {
    let samples_per_pixel: u16 = 1000;
    let scale: f64 = 1.0 / (samples_per_pixel as f64);
    let depth = 50;
    let image_width: u32 = 640;
    let image_height: u32 = 360;
    let aspect_ratio: f64 = 16.0 / 9.0;
    let focal_length: f64 = 1.0;
    let origin = DVec3::new(0.0, 0.0, 0.0);

    let cam = Camera::new(
                    DVec3::new(13.0, 2.0, 3.0),
                    DVec3::new(0.0, 0.0, 0.0),
                    DVec3::new(0.0, 1.0, 0.0),
                    20.0,
                    aspect_ratio,
                    0.0,
                    1.0);

    let world = random_scene();

    let pbar = ProgressBar::new(image_height.into());
    pbar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}]\
            [{wide_bar:.cyan/blue}] {percent}% ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let bufffer: Vec<u8> = (0..image_height).into_par_iter().rev().map(|j|{
        let row: Vec<u8> = (0..image_width).into_par_iter().map(|i|{
            let mut rng = rand::thread_rng();
            let generator = Uniform::from(0.0..1.0);
            let mut color = Color3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let du: f64 = generator.sample(&mut rng);
                let dv: f64 = generator.sample(&mut rng);
                let u = (du + (i as f64)) / ((image_width-1) as f64);
                let v = (dv + (j as f64)) / ((image_height-1) as f64);
                let ray = cam.get_ray(u, v);
                color += ray_color(&ray, &world, depth) * scale;
            }
            let ir = (color.x.sqrt().min(1.0) * 255.0) as u8;
            let ig = (color.y.sqrt().min(1.0) * 255.0) as u8;
            let ib = (color.z.sqrt().min(1.0) * 255.0) as u8;
            vec![ir, ig, ib]
        }).flatten().collect();
        pbar.inc(1);
        row
    }).flatten().collect();

    pbar.finish();

    image::save_buffer(
        "image.png",
        &bufffer,
        image_width,
        image_height,
        image::ColorType::Rgb8).unwrap();
}
